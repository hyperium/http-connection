use http::Version;
use std::net::SocketAddr;
use tokio_tcp::TcpStream;
use HttpConnection;

impl HttpConnection for TcpStream {
    fn version(&self) -> Option<Version> {
        None
    }

    fn remote_addr(&self) -> Option<SocketAddr> {
        self.peer_addr().ok()
    }
}

impl HttpConnection for ::std::net::TcpStream {
    fn version(&self) -> Option<Version> {
        None
    }

    fn remote_addr(&self) -> Option<SocketAddr> {
        self.peer_addr().ok()
    }
}
