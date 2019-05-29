#![doc(html_root_url = "https://docs.rs/http-connection/0.1.0")]
#![deny(missing_debug_implementations, missing_docs, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

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
    fn negotiated_version(&self) -> Option<Version> {
        None
    }

    /// Returns the remote address that this connection is connected to.
    fn remote_addr(&self) -> Option<SocketAddr> {
        None
    }
}
