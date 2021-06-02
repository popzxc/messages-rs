#[cfg_attr(feature = "runtime-tokio", path = "tokio.rs")]
#[cfg_attr(feature = "runtime-async-std", path = "async_std.rs")]
#[cfg_attr(
    not(any(feature = "runtime-tokio", feature = "runtime-async-std")),
    path = "empty.rs"
)]
mod runtime_impl;

pub(crate) use runtime_impl::*;

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_runtime {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature="runtime-tokio", runtime="runtime-async-std"))]
            #[cfg_attr(docsrs, doc(cfg(any(feature="runtime-tokio", runtime="runtime-async-std"))))]
            $item
        )*
    }
}
