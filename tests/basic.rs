extern crate http;
extern crate http_connection;

use http::Version;
use http_connection::HttpConnection;
use std::net::SocketAddr;

pub struct NoValues;

impl HttpConnection for NoValues {
}

pub struct WithVersion;

impl HttpConnection for WithVersion {
    fn version(&self) -> Option<Version> {
        Some(Version::HTTP_11)
    }
}

pub struct WithRemoteAddr;

impl HttpConnection for WithRemoteAddr {
    fn remote_addr(&self) -> Option<SocketAddr> {
        Some("127.0.0.1:3000".parse().unwrap())
    }
}


#[test]
fn no_values() {
    let conn = NoValues;

    assert!(conn.version().is_none());
    assert!(conn.remote_addr().is_none());
}

#[test]
fn version() {
    let conn = WithVersion;

    assert_eq!(Some(Version::HTTP_11), conn.version());
    assert!(conn.remote_addr().is_none());
}

#[test]
fn remote_addr() {
    let conn = WithRemoteAddr;
    let addr = "127.0.0.1:3000".parse().unwrap();

    assert_eq!(Some(addr), conn.remote_addr());
    assert!(conn.version().is_none());
}
