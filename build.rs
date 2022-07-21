use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    // let lnd_rpc_dir_owned;
    // let dir = match std::env::var_os("LND_REPO_DIR") {
    //     Some(lnd_repo_path) => {
    //         let mut lnd_rpc_dir = PathBuf::from(lnd_repo_path);
    //         lnd_rpc_dir.push("lnrpc");
    //         lnd_rpc_dir_owned = lnd_rpc_dir;
    //         &*lnd_rpc_dir_owned
    //     }
    //     None => Path::new("vendor"),
    // };
    //let dir = Path::new("vendor");
    let dirs = &["vendor"];

    let iface_files = &["vendor/lightning.proto", "vendor/invoices.proto"];

    // println!("cargo:rerun-if-changed={}", joined);
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src")
        .compile(iface_files, dirs)?;

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
