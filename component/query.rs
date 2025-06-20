pub trait Search<Q> {
    type Return<'a>
    where
        Self: 'a;
    fn search<'a>(&'a self, query: Q) -> Self::Return<'a>;
}

pub trait Traverse<S> {
    type Iterator<'a>: Iterator
    where
        Self: 'a;

    fn traverse<'a>(&'a self) -> Self::Iterator<'a>;
}

pub trait Strategy: Sized {
    fn traverse<S>(&self) -> <Self as Traverse<S>>::Iterator<'_>
    where
        Self: Traverse<S>,
    {
        <Self as Traverse<S>>::traverse(self)
    }
}

impl<T> Strategy for T {}
