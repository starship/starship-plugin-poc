use futures::{future, StreamExt};
use service::{init_tracing, Plugin};
use std::{env, net::SocketAddr, path::PathBuf, process::Stdio};
use tarpc::{
    context,
    serde_transport::tcp,
    server::{incoming::Incoming, BaseChannel},
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

    let socket_addr = spawn_plugin_server().await?;
    let server_port = socket_addr.port().to_string();

    let mut plugin_handles = Vec::with_capacity(10);
    for _ in 1..10 {
        let mut child = Command::new("./target/debug/starship-plugin-directory")
            .arg(&server_port)
            .stdin(Stdio::null())
            .spawn()?;

        plugin_handles.push(tokio::spawn(async move { child.wait().await }));
    }

    future::join_all(plugin_handles).await;

    Ok(())
}

/// Spawn an instance of the Starship plugin server, providing the socket address
/// for clients to connect to.
async fn spawn_plugin_server() -> anyhow::Result<SocketAddr> {
    let listener = tcp::listen("localhost:0", Bincode::default)
        .await?
        .filter_map(|r| async { r.ok() });

    let socket_addr = listener.get_ref().local_addr();

    let plugin_server = listener
        .map(BaseChannel::with_defaults)
        .execute(PluginServer.serve());
    tokio::spawn(plugin_server);

    Ok(socket_addr)
}
