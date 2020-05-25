use crate::error::Result;
use std::{
    future::Future,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

pub(crate) enum JoinHandle<T> {
    #[cfg(feature = "tokio-runtime")]
    Tokio(tokio::task::JoinHandle<T>),

    #[cfg(feature = "async-std-runtime")]
    AsyncStd(async_std::task::JoinHandle<T>),
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.deref_mut() {
            #[cfg(feature = "tokio-runtime")]
            Self::Tokio(ref mut handle) => Pin::new(handle).poll(cx).map_err(|f| f.into()),

            #[cfg(feature = "async-std-runtime")]
            Self::AsyncStd(ref mut handle) => Pin::new(handle).poll(cx).map(Ok),
        }
    }
}
