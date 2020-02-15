mod bindings;

use std::{
    convert::TryFrom,
    ffi::CString,
    marker::PhantomData,
    mem::ManuallyDrop,
    os::raw::{c_int, c_void},
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};

const NUL_MSG: &str = "valid names shouldn't contain null characters";

pub fn execute<P>(_recipe: Recipe<'_>, mut progress: P) -> Result<Output, Error>
where
    P: FnMut(i32),
{
    // Safety: Accepting a `Recipe` means we prove at compile time that setting
    // these variables can't result in any data races.
    static mut ON_PROGRESS_USER_DATA: *mut c_void = ptr::null_mut();
    static mut TEMPORARY_RESULT: Option<Output> = None;

    unsafe extern "C" fn on_progress<F>(percent: c_int) -> c_int
    where
        F: FnMut(i32),
    {
        // Safety: This requires us to store a pointer to `progress` when
        // `execute()` is called and make sure the `F` type variable
        // `on_progress()` is instantiated with is the same as `execute()`'s `P`
        let actual_progress_callback = &mut *(ON_PROGRESS_USER_DATA as *mut F);

        actual_progress_callback(percent);
        bindings::RESULT_OK as c_int
    }

    unsafe extern "C" fn on_finished(_num_items: c_int) -> c_int {
        let mut output = Output::default();

        let mut item = 0;

        while bindings::stateful_get_output_by_index(
            output.items.len() as c_int,
            &mut item,
        ) == bindings::RESULT_OK as c_int
        {
            output.items.push(item);
        }

        // Safety: Accepting a `Recipe` means this can only be set by one thread
        // at a time
        TEMPORARY_RESULT = Some(output);

        bindings::RESULT_OK as c_int
    }

    unsafe {
        // Safety: Accepting a `Recipe` means this can only be set by one thread
        // at a time
        ON_PROGRESS_USER_DATA = &mut progress as *mut P as *mut c_void;

        let ret = bindings::stateful_execute(
            Some(on_progress::<P>),
            Some(on_finished),
        )
        .into_result();

        // We need to take the temporary result before handling
        // stateful_execute()'s return code so we don't leak an `Output`.
        let output = TEMPORARY_RESULT.take();

        // just bail if something went wrong
        if let Err(e) = ret {
            return Err(e);
        }

        // We need to make sure we actually set the temporary result. The only
        // way this could possibly happen is if `stateful_execute()` ran to
        // completion and said it finished successfully without actually
        // invoking our `on_finished` callback... If so, that's a programming
        // error in the underlying library and nothing the caller can reasonably
        // be expected to handle

        match output {
            Some(output) => Ok(output),
            None => panic!("The stateful_execute function said it returned successfully without calling our on_finished callback. This is a bug.")
        }
    }
}

/// A handle to the `stateful` library.
pub struct Library {
    _not_send: PhantomData<*const ()>,
}

static LIBRARY_IN_USE: AtomicBool = AtomicBool::new(false);

/// A macro you can use when you *know* a function has been statically proven to
/// not fail.
macro_rules! cant_fail {
    ($return_code:expr) => {
        if let Err(e) = $return_code.into_result() {
            unreachable!(
                "The type system should ensure this function can't fail: {}",
                e
            );
        }
    };
}

impl Library {
    pub fn new() -> Result<Library, Error> {
        if LIBRARY_IN_USE.compare_and_swap(false, true, Ordering::SeqCst)
            == false
        {
            unsafe {
                bindings::stateful_open().into_result()?;
            }

            Ok(Library {
                _not_send: PhantomData,
            })
        } else {
            Err(Error::AlreadyInUse)
        }
    }

    /// Start setting parameters.
    ///
    /// The [`SettingParameters`] type uses lifetimes to make sure you can't
    /// use any other [`Library`] functionality while it is alive.
    ///
    /// ```rust,compile_fail
    /// # use stateful_native_library::Library;
    /// let mut library = Library::new().unwrap();
    ///
    /// // start setting parameters
    /// let mut params = library.set_parameters();
    ///
    /// // params is still alive, so trying to create a recipe is a compile
    /// // error
    /// library.create_recipe();
    ///
    /// // params is still alive until here
    /// drop(params);
    /// ```
    pub fn set_parameters(&mut self) -> SettingParameters<'_> {
        cant_fail!(unsafe { bindings::stateful_start_setting_parameters() });
        SettingParameters { _library: self }
    }

    /// Start creating the inputs for [`execute()`].
    ///
    /// The [`RecipeBuilder`] uses lifetimes to make sure you can't do anything
    /// else while creating a [`Recipe`].
    ///
    /// ```rust,compile_fail
    /// # use stateful_native_library::Library;
    /// let mut library = Library::new().unwrap();
    ///
    /// // start creating the recipe
    /// let mut recipe_builder = library.create_recipe();
    ///
    /// // trying to set parameters while recipe_builder is alive is an error
    /// library.set_parameters();
    ///
    /// // recipe_builder is still alive until here
    /// drop(recipe_builder);
    /// ```
    pub fn create_recipe(&mut self) -> RecipeBuilder<'_> {
        cant_fail!(unsafe { bindings::stateful_start_adding_items() });
        RecipeBuilder {
            _library: ManuallyDrop::new(self),
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            let _ = bindings::stateful_close();
        }
        LIBRARY_IN_USE.store(false, Ordering::SeqCst);
    }
}

pub struct SettingParameters<'lib> {
    _library: &'lib mut Library,
}

impl<'lib> SettingParameters<'lib> {
    pub fn boolean(&mut self, name: &str, value: bool) -> &mut Self {
        let name = CString::new(name).expect(NUL_MSG);

        unsafe {
            cant_fail!(bindings::stateful_set_bool_var(name.as_ptr(), value));
        }

        self
    }

    pub fn integer(&mut self, name: &str, value: i32) -> &mut Self {
        let name = CString::new(name).expect(NUL_MSG);

        unsafe {
            cant_fail!(bindings::stateful_set_int_var(name.as_ptr(), value));
        }

        self
    }
}

impl<'lib> Drop for SettingParameters<'lib> {
    fn drop(&mut self) {
        unsafe {
            let _ = bindings::stateful_end_setting_parameters().into_result();
        }
    }
}

pub struct RecipeBuilder<'lib> {
    // Note: We use ManuallyDrop::take() so we can move the `&mut Library`
    // reference out of `RecipeBuilder` even though it has a Drop impl.
    //
    // This is okay because a mutable reference doesn't need to do anything
    // special when it goes out of scope
    //
    // https://users.rust-lang.org/t/moving-out-of-a-type-implementing-drop/38225/5?u=michael-f-bryan
    _library: ManuallyDrop<&'lib mut Library>,
}

impl<'lib> RecipeBuilder<'lib> {
    pub fn add_item(&mut self, name: &str, value: i32) -> &mut Self {
        let name = CString::new(name).expect(NUL_MSG);
        cant_fail!(unsafe {
            bindings::stateful_add_item(name.as_ptr(), value)
        });

        self
    }

    pub fn add_group<'r>(&'r mut self, name: &str) -> GroupBuilder<'r, 'lib> {
        let name = CString::new(name).expect(NUL_MSG);
        cant_fail!(unsafe {
            bindings::stateful_start_adding_group(name.as_ptr())
        });
        GroupBuilder {
            _recipe_builder: ManuallyDrop::new(self),
        }
    }

    pub fn build(mut self) -> Recipe<'lib> {
        // Safety: RecipeBuilder's Drop impl does not use this field
        let _library = unsafe { ManuallyDrop::take(&mut self._library) };
        Recipe { _library }
    }
}

impl<'lib> Drop for RecipeBuilder<'lib> {
    fn drop(&mut self) {
        unsafe {
            let _ = bindings::stateful_end_adding_items().into_result();
        }
    }
}

pub struct GroupBuilder<'r, 'lib> {
    _recipe_builder: ManuallyDrop<&'r mut RecipeBuilder<'lib>>,
}

impl<'r, 'lib> GroupBuilder<'r, 'lib> {
    pub fn add_item(&mut self, name: &str, value: i32) -> &mut Self {
        let name = CString::new(name).expect(NUL_MSG);
        cant_fail!(unsafe {
            bindings::stateful_add_group_item(name.as_ptr(), value)
        });

        self
    }

    pub fn finish(mut self) -> &'r mut RecipeBuilder<'lib> {
        unsafe {
            // Safety: GroupBuilder's Drop impl does not use this field
            ManuallyDrop::take(&mut self._recipe_builder)
        }
    }
}

impl<'r, 'lib> Drop for GroupBuilder<'r, 'lib> {
    fn drop(&mut self) {
        unsafe {
            let _ = bindings::stateful_end_adding_group().into_result();
        }
    }
}

pub struct Recipe<'lib> {
    _library: &'lib mut Library,
}

/// The various error cases that may be encountered while using this library.
#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("The library is already in use")]
    AlreadyInUse,
    #[error("The underlying library is in an invalid state")]
    InvalidState,
    #[error("An argument was invalid")]
    InvalidArgument,
    #[error("Unknown error code: {}", _0)]
    Other(c_int),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Output {
    pub items: Vec<i32>,
}

trait IntoResult {
    fn into_result(self) -> Result<(), Error>;
}

impl IntoResult for c_int {
    fn into_result(self) -> Result<(), Error> {
        let code = u32::try_from(self).map_err(|_| Error::Other(self))?;

        match code {
            bindings::RESULT_OK => Ok(()),
            bindings::RESULT_BAD_STATE => Err(Error::InvalidState),
            bindings::RESULT_INVALID_ARGUMENT => Err(Error::InvalidArgument),
            _ => Err(Error::Other(self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_not_impl_any!(Library: Send, Sync);

    #[test]
    fn cant_create_multiple_library_handles_at_the_same_time() {
        let first_library = Library::new().unwrap();

        // make sure the flag is set
        assert!(LIBRARY_IN_USE.load(Ordering::SeqCst));

        // then try to create another handle
        assert!(Library::new().is_err());

        // explicitly drop the first library so we know it clears the flag
        drop(first_library);

        assert!(!LIBRARY_IN_USE.load(Ordering::SeqCst));

        // now the old handle is destroyed, we can create another
        let _another = Library::new().unwrap();
    }

    #[test]
    fn ffi_bindings_smoke_test() {
        unsafe {
            assert_eq!(bindings::stateful_open(), bindings::RESULT_OK as c_int);
            assert_eq!(
                bindings::stateful_close(),
                bindings::RESULT_OK as c_int
            );
        }
    }
}
