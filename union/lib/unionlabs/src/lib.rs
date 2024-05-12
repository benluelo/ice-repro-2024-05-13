use core::marker::PhantomData;

pub trait Baz: Sized {
    type Error;
    fn baz() -> Result<Self, Self::Error>;
}

pub struct Foo<T>(pub T);

pub enum FooError<T> {
    BazError(T),
}

impl<T> Baz for Foo<T> {
    type Error = WrapperError<FooError<T>>;
    fn baz() -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct Bar<T> {
    __marker: PhantomData<T>,
}

pub enum BarError<T: Baz> {
    __Marker(PhantomData<T>),
}

impl<T: Baz> Baz for Bar<T> {
    type Error = BarError<T>;
    fn baz() -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum WrapperError<E> {
    TryFromProto(E),
}
