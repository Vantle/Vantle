pub trait Stateful<T> {
    fn scale(&self, basis: &T) -> usize;
}
