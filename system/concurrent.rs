pub use error;

pub trait Join {
    type Output;

    fn join(self) -> impl std::future::Future<Output = error::Result<Self::Output>> + Send;
}

pub trait Schedule {
    type Task<T: Send + 'static>: Join<Output = T>;

    fn submit<F>(&self, future: F) -> Self::Task<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static;

    fn execute<F, T>(&self, function: F) -> Self::Task<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;

    fn block<F: std::future::Future>(&self, future: F) -> F::Output;
}
