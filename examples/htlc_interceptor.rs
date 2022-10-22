///
/// To use this demo:
/// - create a 3 node network in polar (https://lightningpolar.com/) -- Alice, Bob and Carol
/// - Create channels between Alice and Carol, and between Alice and Bob.
/// - Create an invoice from Carol. Use the `decodepayreq` and `lookupinvoice` lncli commands to get preimage associated with Carol's invoice
/// - Run this program with Alice's LND credentials, and the preimage from step above. For instance:
///   `cargo run --example htlc_interceptor 127.0.0.1 10003 /Users/justin/.polar/networks/2/volumes/lnd/alice/tls.cert /Users/justin/.polar/networks/2/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon <preimage>`
/// - Have Bob pay the invoice via Polar UI
/// - This program should output HTLC information. Polar should print success message.
///

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let host = args
        .next()
        .expect("missing arguments: host, port, cert file, macaroon file, preimage");
    let port = args
        .next()
        .expect("missing arguments: port, cert file, macaroon file, preimage");
    let cert_file = args
        .next()
        .expect("missing arguments: cert file, macaroon file, preimage");
    let macaroon_file = args
        .next()
        .expect("missing arguments: macaroon file, preimage");
    let preimage = args.next().expect("missing argument: preimage");
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
    let preimage = hex::decode(preimage.into_string().expect("preimage is invalid UTF-8")).unwrap();

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_openssl_lnd::connect_router(host, port, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let (tx, rx) = tokio::sync::mpsc::channel::<
        tonic_openssl_lnd::routerrpc::ForwardHtlcInterceptResponse,
    >(1024);
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    let mut htlc_stream = client
        .htlc_interceptor(stream)
        .await
        .expect("Failed to call subscribe_invoices")
        .into_inner();

    while let Some(htlc) = htlc_stream
        .message()
        .await
        .expect("Failed to receive invoices")
    {
        println!("htlc {:?}", htlc);
        let response = tonic_openssl_lnd::routerrpc::ForwardHtlcInterceptResponse {
            incoming_circuit_key: htlc.incoming_circuit_key,
            action: 0,                  // this will claim the htlc
            preimage: preimage.clone(), // this would be for a real preimage
            failure_code: 0,
            failure_message: vec![],
        };
        tx.send(response).await.unwrap();
    }
}
