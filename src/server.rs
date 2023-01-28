// ! The server's responsibilities are the following:
// ! - Manage the lifecycle of the plugin processes
// ! - Maintain an in-memory cache of previous prompt calculations
// ! - Provide an interface for async prompts to continue being calculated

use futures::{future, StreamExt};
use service::{init_tracing, Plugin};
use std::{env, net::SocketAddr, path::PathBuf, process::Stdio};
use tarpc::{
    context,
    serde_transport::tcp,
    server::{incoming::Incoming, BaseChannel, Channel},
};
use tokio::process::Command;
use tokio_serde::formats::Bincode;

#[derive(Clone)]
struct PluginServer;

#[tarpc::server]
impl Plugin for PluginServer {
    async fn current_dir(self, _: context::Context) -> PathBuf {
        env::current_dir().expect("could not retreive current dir")
    }
    async fn output(self, _: context::Context, output: String) {
        println!("{}", output);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;
    spawn_plugin_server().await?;

    Ok(())
}

/// Spawn an instance of the Starship plugin server, providing the socket address
/// for clients to connect to.
async fn spawn_plugin_server() -> anyhow::Result<()> {
    let listener = tcp::listen("localhost:0", Bincode::default).await?;

    let socket_addr = listener.local_addr();
    println!("Listening on {}", socket_addr);

    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(BaseChannel::with_defaults)
        .map(|channel| channel.execute(PluginServer.serve()))
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
