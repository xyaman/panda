use std::{
    future::Future,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

pub(crate) enum Delay {
    #[cfg(feature = "tokio-runtime")]
    Tokio(tokio::time::Delay),

    #[cfg(feature = "async-std-runtime")]
    AsyncStd(Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>),
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.deref_mut() {
            #[cfg(feature = "tokio-runtime")]
            Self::Tokio(ref mut delay) => Pin::new(delay).poll(cx),

            #[cfg(feature = "async-std-runtime")]
            Self::AsyncStd(ref mut handle) => Pin::new(handle).poll(cx),
        }
    }
}
