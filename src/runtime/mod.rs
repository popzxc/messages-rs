#[cfg_attr(
    all(feature = "runtime-tokio", not(feature = "runtime-async-std")),
    path = "tokio.rs"
)]
#[cfg_attr(
    all(feature = "runtime-async-std", not(feature = "runtime-tokio")),
    path = "async_std.rs"
)]
#[cfg_attr(
    not(any(feature = "runtime-tokio", feature = "runtime-async-std",)),
    path = "empty.rs"
)]
mod runtime_impl;

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_runtime {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature="runtime-tokio", feature="runtime-async-std"))]
            #[cfg_attr(docsrs, doc(cfg(any(feature = "runtime-tokio", feature="runtime-async-std"))))]
            $item
        )*
    }
}

cfg_runtime! {
    pub use runtime_impl::JoinHandle;
    pub(crate) use runtime_impl::*;
}
