use unionlabs::{Bar, Baz, Foo};

pub trait Trait {
    type Quux: Baz;
}

pub enum BazError<T: Trait> {
    FooBarQuux(<Foo<Bar<T::Quux>> as Baz>::Error),
}

// ICE: Visibility MUST be `pub`
// NOTE: Renaming this apparently affects whatever graph is built internally, renaming it can cause compilation to succeed
pub mod storage_utils {
    use unionlabs::{Bar, Baz, Foo};

    use crate::{BazError, Trait};

    pub fn called_fn<T>() -> Result<Bar<T::Quux>, BazError<T>>
    where
        // ICE: Removing this (redundant) bound causes it to compile
        Foo<Bar<T::Quux>>: Baz,
        T: Trait,
    {
        // ICE HERE
        broken_fn::<T>()
    }

    fn broken_fn<T>() -> Result<Bar<T::Quux>, BazError<T>>
    where
        T: Trait,
    {
        // ICE HERE
        <Foo<Bar<T::Quux>>>::baz()
            .map(|any| any.0)
            .map_err(BazError::<T>::FooBarQuux)
    }
}
