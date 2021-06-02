use super::JoinHandle;
use std::future::Future;

pub(crate) fn spawn<T>(task: T) -> impl Future<Output = T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    async_std::task::spawn(task)
}
