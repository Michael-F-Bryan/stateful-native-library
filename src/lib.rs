mod bindings;

use std::{
    convert::TryFrom,
    marker::PhantomData,
    os::raw::c_int,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn execute<P>(_recipe: Recipe<'_>, _progress: P) -> Result<Output, Error>
where
    P: FnMut(i32),
{
    unimplemented!()
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
        RecipeBuilder { _library: self }
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
    pub fn boolean(&mut self, _name: &str, _value: bool) -> &mut Self {
        unimplemented!()
    }

    pub fn integer(&mut self, _name: &str, _value: i32) -> &mut Self {
        unimplemented!()
    }
}

pub struct RecipeBuilder<'lib> {
    _library: &'lib mut Library,
}

impl<'lib> RecipeBuilder<'lib> {
    pub fn add_item(&mut self, _name: &str, _value: i32) -> &mut Self {
        unimplemented!()
    }

    pub fn add_group<'r>(&'r mut self, _name: &str) -> GroupBuilder<'r, 'lib> {
        GroupBuilder {
            _recipe_builder: self,
        }
    }

    pub fn build(self) -> Recipe<'lib> {
        Recipe {
            _library: self._library,
        }
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
    _recipe_builder: &'r mut RecipeBuilder<'lib>,
}

impl<'r, 'lib> GroupBuilder<'r, 'lib> {
    pub fn add_item(&mut self, _name: &str, _value: i32) -> &mut Self {
        unimplemented!()
    }

    pub fn finish(self) -> &'r mut RecipeBuilder<'lib> { self._recipe_builder }
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
