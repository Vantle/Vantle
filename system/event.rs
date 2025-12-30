pub trait Sink<T> {
    fn react(&mut self, event: &T);
}

pub trait Source<T> {
    fn interact(&mut self, sink: Box<dyn Sink<T>>);
    fn act(&mut self, event: &T);
}
