use std::future::Future;

pub(crate) fn spawn<T>(_task: T) -> !
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    panic!("Task spawning is not available without runtime feature")
}
