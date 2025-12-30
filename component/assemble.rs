pub trait Assemble {
    type Output;

    fn assemble(self) -> Self::Output;
}

pub trait Async {
    type Output;

    fn assemble(self) -> impl std::future::Future<Output = Self::Output> + Send;
}
