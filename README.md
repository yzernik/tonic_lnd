# Tonic LND client

Rust implementation of LND RPC client using async gRPC library `tonic_openssl`.

## About

**Warning: this crate is in early development and may have unknown problems!
Review it before using with mainnet funds!**

This crate implements LND GRPC using [`tonic_openssl`](https://docs.rs/tonic-openssl/latest/tonic_openssl/) and [`prost`](https://docs.rs/prost/).
Apart from being up-to-date at the time of writing (:D) it also allows `async` usage.
It contains vendored `*.proto` files so LND source code is not *required*
but accepts an environment variable `LND_REPO_DIR` which overrides the vendored `*.proto` files.
This can be used to test new features in non-released `lnd`.
(Actually, the motivating project using this library was that case. :))

## Usage

There's no setup needed beyond adding the crate to your `Cargo.toml`.
If you need to change the `*.proto` files from which the client is generated, set the environment variable `LND_REPO_DIR` to a directory with cloned [`lnd`](https://github.com/lightningnetwork/lnd.git) during build.

Here's an example of retrieving information from LND (`[getinfo](https://api.lightning.community/#getinfo)` call).
You can find the same example in crate root for your convenience.

```rust
// This program accepts four arguments: host, port, cert file, macaroon file

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let host = args
        .next()
        .expect("missing arguments: host, port, cert file, macaroon file");
    let port = args
        .next()
        .expect("missing arguments: port, cert file, macaroon file");
    let cert_file = args
        .next()
        .expect("missing arguments: cert file, macaroon file");
    let macaroon_file = args.next().expect("missing argument: macaroon file");
    let host: String = host.into_string().expect("host is not UTF-8");
    let port: u32 = port
        .into_string()
        .expect("port is not UTF-8")
        .parse()
        .expect("port is not u32");
    let cert_file: String = cert_file.into_string().expect("cert_file is not UTF-8");
    let macaroon_file: String = macaroon_file
        .into_string()
        .expect("macaroon_file is not UTF-8");

    // Connecting to LND requires only host, port, cert file, macaroon file
    let mut client = tonic_openssl_lnd::connect(host, port, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let info = client
        .lightning()
        // All calls require at least empty parameter
        .get_info(tonic_openssl_lnd::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
```

## License

MITNFA
