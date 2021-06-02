use std::future::Future;

use futures::FutureExt;

pub(crate) fn spawn<T>(task: T) -> impl Future<Output = T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(task).map(|res| res.expect("Unable to join future"))
}
