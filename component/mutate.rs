pub trait Insert<T> {
    type Return<'a>
    where
        Self: 'a;
    fn insert<'a>(&'a mut self, value: T) -> Self::Return<'a>;
}

pub trait Delete<T> {
    type Return<'a>
    where
        Self: 'a;
    fn delete<'a>(&'a mut self, value: T) -> Self::Return<'a>;
}
