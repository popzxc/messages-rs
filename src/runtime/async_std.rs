use std::future::Future;

pub fn spawn<T>(task: T)
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    // TODO: Should we return the join handle?
    async_std::task::spawn(task);
}
