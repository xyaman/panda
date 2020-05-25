mod delay;
mod join_handle;
pub(crate) mod websocket;

use delay::Delay;
use join_handle::JoinHandle;

use std::{future::Future, time::Duration};

pub(crate) fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    #[cfg(feature = "tokio-runtime")]
    return JoinHandle::Tokio(tokio::spawn(future));

    #[cfg(feature = "async-std-runtime")]
    return JoinHandle::AsyncStd(async_std::task::spawn(future));
}

pub(crate) fn sleep(duration: Duration) -> delay::Delay {
    #[cfg(feature = "tokio-runtime")]
    return Delay::Tokio(tokio::time::delay_for(duration));

    #[cfg(feature = "async-std-runtime")]
    return Delay::AsyncStd(Box::pin(async_std::task::sleep(duration)));
}
