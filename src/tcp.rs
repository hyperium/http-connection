use http::Version;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use HttpConnection;

impl HttpConnection for TcpStream {
    fn negotiated_version(&self) -> Option<Version> {
        None
    }

    fn remote_addr(&self) -> Option<SocketAddr> {
        self.peer_addr().ok()
    }
}

impl HttpConnection for ::std::net::TcpStream {
    fn negotiated_version(&self) -> Option<Version> {
        None
    }

    fn remote_addr(&self) -> Option<SocketAddr> {
        self.peer_addr().ok()
    }
}
