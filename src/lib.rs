mod bindings;

use std::{
    marker::PhantomData,
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

impl Library {
    pub fn new() -> Result<Library, AlreadyInUse> {
        if LIBRARY_IN_USE.compare_and_swap(false, true, Ordering::SeqCst)
            == false
        {
            Ok(Library {
                _not_send: PhantomData,
            })
        } else {
            Err(AlreadyInUse)
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
        RecipeBuilder { _library: self }
    }
}

impl Drop for Library {
    fn drop(&mut self) { LIBRARY_IN_USE.store(false, Ordering::SeqCst); }
}

/// An error indicating the `Library` can't be accessed again until the existing
/// session is over.
#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
#[error("The library is already in use")]
pub struct AlreadyInUse;

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

pub struct GroupBuilder<'r, 'lib> {
    _recipe_builder: &'r mut RecipeBuilder<'lib>,
}

impl<'r, 'lib> GroupBuilder<'r, 'lib> {
    pub fn add_item(&mut self, _name: &str, _value: i32) -> &mut Self {
        unimplemented!()
    }

    pub fn finish(self) -> &'r mut RecipeBuilder<'lib> { self._recipe_builder }
}

pub struct Recipe<'lib> {
    _library: &'lib mut Library,
}

pub enum Error {}

pub struct Output {
    pub items: Vec<i32>,
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
}
