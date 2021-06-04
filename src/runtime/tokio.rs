use std::future::Future;

pub use tokio::task::JoinHandle;

pub(crate) fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(task)
}
