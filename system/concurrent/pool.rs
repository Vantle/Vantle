use assemble::Assemble;
use tracing::Instrument;

pub struct Schedule {
    inner: tokio::runtime::Runtime,
}

impl concurrent::Schedule for Schedule {
    type Task<T: Send + 'static> = Task<T>;

    fn submit<F>(&self, future: F) -> Task<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let span = tracing::Span::current();
        Task(self.inner.spawn(future.instrument(span)))
    }

    fn execute<F, T>(&self, function: F) -> Task<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let span = tracing::Span::current();
        Task(self.inner.spawn_blocking(move || {
            let _enter = span.enter();
            function()
        }))
    }

    fn block<F: std::future::Future>(&self, future: F) -> F::Output {
        self.inner.block_on(future)
    }
}

impl Schedule {
    pub fn context(&self) -> tokio::runtime::EnterGuard<'_> {
        self.inner.enter()
    }
}

pub struct Task<T>(tokio::task::JoinHandle<T>);

impl<T: Send + 'static> concurrent::Join for Task<T> {
    type Output = T;

    async fn join(self) -> error::Result<T> {
        self.0.await.map_err(|source| error::Error::Join { source })
    }
}

pub struct Assembler {
    workers: std::num::NonZeroUsize,
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            workers: std::thread::available_parallelism()
                .unwrap_or(std::num::NonZeroUsize::new(4).unwrap_or(std::num::NonZeroUsize::MIN)),
        }
    }

    #[must_use]
    pub fn workers(mut self, count: std::num::NonZeroUsize) -> Self {
        self.workers = count;
        self
    }
}

impl Assemble for Assembler {
    type Output = error::Result<Schedule>;

    fn assemble(self) -> Self::Output {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.workers.get())
            .enable_all()
            .build()
            .map(|inner| Schedule { inner })
            .map_err(|source| error::Error::Activate { source })
    }
}
