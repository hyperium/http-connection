//! Asynchronous HTTP connection
//!
//! This trait decorates an `AsyncRead + AsyncWrite` connection stream/sink with HTTP
//! aware information like the connections HTTP version and the remote address.

extern crate http;
#[cfg(feature = "tcp")]
extern crate tokio_tcp;

use http::Version;
use std::net::SocketAddr;

#[cfg(feature = "tcp")]
mod tcp;

/// Represents a HTTP aware connection.
///
/// This connection is a `AsyncRead + AsyncWrite` stream that provides information
/// on what http versions were determinted `ALPN` negotiation or what the remote address
/// this stream is connected too.
pub trait HttpConnection {
    /// Returns the version that this stream is set too.
    ///
    /// For `version` this indicates that this stream is accepting http frames of the version
    /// returned. If `None` is returned then there has been no prior negotiation for the http
    /// version.
    fn version(&self) -> Option<Version>;

    /// Returns the remote address that this connection is connected to.
    fn remote_addr(&self) -> Option<SocketAddr>;
}
