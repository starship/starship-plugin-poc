use anyhow::Result;
use futures::{future, StreamExt};
use once_cell::sync::Lazy;
use plugin_runtime::Runtime;
use starship_service::StarshipService;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tarpc::context::Context;
use tarpc::serde_transport::tcp;
use tarpc::server::{self, Channel};
use tarpc::tokio_serde::formats::Bincode;

mod plugin_runtime;

#[derive(Clone)]
struct StarshipServer(SocketAddr);

#[tarpc::server]
impl StarshipService for StarshipServer {
    async fn prompt(self, _: Context) -> String {
        render_prompt().await.unwrap()
    }
}

pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    let wasm_bytes =
        std::fs::read("./target/wasm32-unknown-unknown/debug/starship_plugin_directory.wasm")
            .unwrap();
    Runtime::new(wasm_bytes).unwrap()
});

async fn render_prompt() -> Result<String> {
    let output = RUNTIME.output().await;
    output.map_err(Into::into)
}

// pub static CACHE_DIR: Lazy<PathBuf> =
//     Lazy::new(|| PathBuf::from(tilde("~/.cache/starship").into_owned()));
// pub static CONFIG_DIR: Lazy<PathBuf> =
//     Lazy::new(|| PathBuf::from(tilde("~/.config/starship").into_owned()));
// pub static DATA_DIR: Lazy<PathBuf> =
//     Lazy::new(|| PathBuf::from(tilde("~/.local/share/starship").into_owned()));

// pub static SOCKET_PATH: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("starship.sock"));

// fn _init_directories() -> Result<()> {
//     if !DATA_DIR.exists() {
//         fs::create_dir_all(DATA_DIR.as_path())?;
//     }

//     if !CONFIG_DIR.exists() {
//         fs::create_dir_all(CONFIG_DIR.as_path())?;
//     }

//     if !CACHE_DIR.exists() {
//         fs::create_dir_all(CACHE_DIR.as_path())?;
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize runtime
    let _ = RUNTIME.metadata();

    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 8000);

    let listener = tcp::listen(&server_addr, Bincode::default).await?;

    println!("Listening on port {}", listener.local_addr().port());

    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = StarshipServer(channel.transport().peer_addr().unwrap());
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
