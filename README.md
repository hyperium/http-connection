# HTTP Connection

A trait representing asynchronous HTTP connection.

[![Build Status][azure-badge]][azure-url]

[azure-badge]: https://dev.azure.com/hyperium/http-connection/_apis/build/status/hyperium.http-connection?branchName=master
[azure-url]: https://dev.azure.com/hyperium/http-connection/_build/latest?definitionId=1&branchName=master

More information about this crate can be found in the [crate
documentation][dox].

[dox]: https://docs.rs/http-connection

## Usage

To use `http-connection`, first add this to your `Cargo.toml`:

```toml
[dependencies]
http-connection = "0.1.0"
```

Next, add this to your crate:

```rust
use http_connection::HttpConnection;

fn main() {
    // ...
}
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `http-connection` by you, shall be licensed as MIT, without any additional
terms or conditions.
