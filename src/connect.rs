//! Connection helper.
use wstd::net::TcpStream;

use tungstenite::{
    error::{Error, UrlError},
    handshake::client::{Request, Response},
    protocol::WebSocketConfig,
};

use crate::{IntoClientRequest, WebSocketStream, client_async_with_config, domain};

/// Connect to a given URL.
///
/// Accepts any request that implements [`IntoClientRequest`], which is often just `&str`, but can
/// be a variety of types such as `httparse::Request` or [`tungstenite::http::Request`] for more
/// complex uses.
///
/// URLs that start with `wss://` are not supported since `wstd` doesn't support TLS.
///
/// ```no_run
/// # use tungstenite::client::IntoClientRequest;
///
/// # async fn test() {
/// use tungstenite::http::{Method, Request};
/// use wstd_tungstenite::connect_async;
///
/// let mut request = "ws://localhost:8080".into_client_request().unwrap();
///
/// let (stream, response) = connect_async(request).await.unwrap();
/// # }
/// ```
pub async fn connect_async<R>(request: R) -> Result<(WebSocketStream<TcpStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    connect_async_with_config(request, None).await
}

/// The same as `connect_async()` but the one can specify a websocket configuration.
/// Please refer to `connect_async()` for more details. `disable_nagle` specifies if
/// the Nagle's algorithm must be disabled, i.e. `set_nodelay(true)`. If you don't know
/// what the Nagle's algorithm is, better leave it set to `false`.
pub async fn connect_async_with_config<R>(
    request: R,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<TcpStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    connect(request.into_client_request()?, config).await
}

async fn connect(
    request: Request,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<TcpStream>, Response), Error> {
    let domain = domain(&request)?;
    let port = request
        .uri()
        .port_u16()
        .or_else(|| match request.uri().scheme_str() {
            Some("ws") => Some(80),
            // we don't support TLS, so fail if `wss://` scheme is used
            _ => None,
        })
        .ok_or(Error::Url(UrlError::UnsupportedUrlScheme))?;

    let addr = format!("{domain}:{port}");
    let socket = TcpStream::connect(addr).await.map_err(Error::Io)?;

    client_async_with_config(request, socket, config).await
}
