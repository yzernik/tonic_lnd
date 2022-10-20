use futures_util::stream;
use rand::rngs::ThreadRng;
use rand::Rng;

// This program connects to LND and prints out intercepted htlc's.
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

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_openssl_lnd::connect_router(host, port, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let mut rng = rand::thread_rng();
    let htlc_count: i32 = rng.gen_range(2..100);

    let mut htlcs = vec![];
    for _ in 0..=htlc_count {
        htlcs.push(random_htlc(&mut rng))
    }

    let mut intercept_stream = client
        .htlc_interceptor(stream::iter(htlcs))
        .await
        .expect("Failed to call htlc_interceptor")
        .into_inner();

    while let Some(intercept) = intercept_stream
        .message()
        .await
        .expect("Failed to receive intercepts")
    {
        println!("{:?}", intercept);
    }
}

fn random_htlc(rng: &mut ThreadRng) -> tonic_openssl_lnd::routerrpc::ForwardHtlcInterceptResponse {
    let chan_id = (rng.gen_range(0..180) - 90) * 10_000_000;
    let htlc_id = (rng.gen_range(0..360) - 180) * 10_000_000;
    let circuit_key = tonic_openssl_lnd::routerrpc::CircuitKey {
        chan_id: chan_id,
        htlc_id: htlc_id,
    };
    let preimage = vec![0; 32];

    tonic_openssl_lnd::routerrpc::ForwardHtlcInterceptResponse {
        incoming_circuit_key: Some(circuit_key),
        action: tonic_openssl_lnd::routerrpc::ResolveHoldForwardAction::Settle.into(),
        preimage: preimage,
        ..Default::default()
    }
}
