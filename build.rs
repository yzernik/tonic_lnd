use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    let lnd_rpc_dir_owned;
    let dir_path = match std::env::var_os("LND_REPO_DIR") {
        Some(lnd_repo_path) => {
            let mut lnd_rpc_dir = PathBuf::from(lnd_repo_path);
            lnd_rpc_dir.push("lnrpc");
            lnd_rpc_dir_owned = lnd_rpc_dir;
            &*lnd_rpc_dir_owned
        }
        None => Path::new("vendor"),
    };
    let dir = dir_path.display().to_string();

    let protos = vec![
        // "autopilot.proto",
        // "chainnotifier.proto",
        // "dev.proto",
        "invoices.proto",
        "lightning.proto",
        // "lncli.proto",
        // "neutrino.proto",
        // "peers.proto",
        // "router.proto",
        // "signer.proto",
        // "stateservice.proto",
        // "verrpc.proto",
        // "walletkit.proto",
        // "walletunlocker.proto",
        // "watchtower.proto",
        // "wtclient.proto",
    ];

    let proto_paths: Vec<_> = protos
        .iter()
        .map(|proto| {
            let mut path = PathBuf::from(&dir);
            path.push(proto);
            path.display().to_string()
        })
        .collect();

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/gen")
        .compile(&proto_paths, &[dir])?;

    // for proto_file in [
    //     "autopilot.proto",
    //     "chainnotifier.proto",
    //     "dev.proto",
    //     "invoices.proto",
    //     "lightning.proto",
    //     "lncli.proto",
    //     "neutrino.proto",
    //     "peers.proto",
    //     "router.proto",
    //     "signer.proto",
    //     "stateservice.proto",
    //     "verrpc.proto",
    //     "walletkit.proto",
    //     "walletunlocker.proto",
    //     "watchtower.proto",
    //     "wtclient.proto",
    // ] {
    // }
    Ok(())
}
