use std::{
    future::Future,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

/// Used with runtime::sleep()
pub(crate) enum Delay {
    #[cfg(feature = "tokio-runtime")]
    Tokio(tokio::time::Delay),

    #[cfg(feature = "async-std-runtime" /*, feature = "async-std-native-tls"*/)]
    AsyncStd(Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>),
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.deref_mut() {
            #[cfg(feature = "tokio-runtime")]
            Self::Tokio(ref mut delay) => Pin::new(delay).poll(cx),

            #[cfg(feature = "async-std-runtime" /*, feature = "async-std-native-tls"*/)]
            Self::AsyncStd(ref mut handle) => Pin::new(handle).poll(cx),
        }
    }
}
