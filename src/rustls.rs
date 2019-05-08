use http::Version;
use std::net::SocketAddr;
use tokio_rustls::TlsStream;
use HttpConnection;

impl<IO, S> HttpConnection for TlsStream<IO, S>
where
    IO: HttpConnection,
{
    fn version(&self) -> Option<Version> {
        self.get_ref().0.version()
    }

    fn remote_addr(&self) -> Option<SocketAddr> {
        self.get_ref().0.remote_addr()
    }
}
