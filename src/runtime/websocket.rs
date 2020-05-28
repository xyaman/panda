use crate::error::Result;
use async_tungstenite::{stream::Stream, WebSocketStream, tungstenite::Message};
use isahc::http;
use url::Url;

// ******************************
// TOKIO
// ******************************
#[cfg(feature = "tokio-runtime")]
use async_tungstenite::tokio::TokioAdapter;

#[cfg(feature = "tokio-runtime")]
pub(crate) type WebSocket = WebSocketStream<
    Stream<
        TokioAdapter<tokio::net::TcpStream>,
        TokioAdapter<tokio_tls::TlsStream<TokioAdapter<TokioAdapter<tokio::net::TcpStream>>>>,
    >,
>;
#[cfg(feature="tokio-runtime")]
pub(crate) type WebSocketSender = futures::stream::SplitSink<WebSocketStream<Stream<TokioAdapter<tokio::net::TcpStream>, TokioAdapter<tokio_tls::TlsStream<TokioAdapter<TokioAdapter<tokio::net::TcpStream>>>>>>, Message>;

#[cfg(feature = "tokio-runtime")]
pub(crate) async fn connect_async(url: Url) -> Result<(WebSocket, http::Response<()>)> {
    async_tungstenite::tokio::connect_async(url).await.map_err(|e| e.into())
}

// ******************************
// ASYNC-STD
// *****************************
#[cfg(feature = "async-std-runtime")]
pub(crate) type WebSocket = WebSocketStream<Stream<async_std::net::TcpStream, async_tls::client::TlsStream<async_std::net::TcpStream>>>;

#[cfg(feature = "async-std-runtime")]
pub(crate) type WebSocketSender = futures::stream::SplitSink<WebSocketStream<Stream<async_std::net::TcpStream, async_tls::client::TlsStream<async_std::net::TcpStream>>>, Message>;

#[cfg(feature = "async-std-runtime")]
pub(crate) async fn connect_async(url: Url) -> Result<(WebSocket, http::Response<()>)> {
    async_tungstenite::async_std::connect_async(url).await.map_err(|e| e.into())
}


// ******************************
// ASYNC-STD(NATIVE TLS)
// *****************************
// #[cfg(feature = "async-std-native-tls")]
// pub(crate) type WebSocket = WebSocketStream<Stream<async_std::net::TcpStream, async_native_tls::TlsStream<async_std::net::TcpStream>>>;

// #[cfg(feature = "async-std-native-tls")]
// pub(crate) type WebSocketSender = futures::stream::SplitSink<WebSocketStream<Stream<async_std::net::TcpStream, async_native_tls::TlsStream<async_std::net::TcpStream>>>, Message>;

// #[cfg(feature = "async-std-native-tls")]
// pub(crate) async fn connect_async(url: Url) -> Result<(WebSocket, http::Response<()>)> {
//     async_tungstenite::async_std::connect_async(url).await.map_err(|e| e.into())
// } 