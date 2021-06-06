use std::future::Future;

pub use async_std::task::JoinHandle;

pub(crate) fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    async_std::task::spawn(task)
}
