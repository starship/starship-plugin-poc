use futures::{future, StreamExt};
use interprocess::nonblocking::local_socket::LocalSocketListener;
use service::{init_tracing, Plugin};
use std::{env::current_dir, path::PathBuf, process::Stdio};
use tarpc::{
    context,
    serde_transport::Transport,
    server::{incoming::Incoming, BaseChannel},
};
use tokio::process::Command;
use tokio_serde::formats::Bincode;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[derive(Clone)]
struct PluginServer;

#[tarpc::server]
impl Plugin for PluginServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {}!", name)
    }
    async fn current_dir(self, _: context::Context) -> Result<PathBuf, String> {
        current_dir().map_err(|e| format!("{:?}", e))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;

    let plugin_list = vec![
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
        "./target/debug/client",
    ];

    spawn_plugin_server().await?;

    let mut plugin_handles = Vec::with_capacity(plugin_list.len());
    for plugin in plugin_list {
        let mut child = Command::new(plugin).stdin(Stdio::null()).spawn()?;

        plugin_handles.push(tokio::spawn(async move { child.wait().await }));
    }

    future::join_all(plugin_handles).await;

    Ok(())
}

/// Spawn an instance of the Starship plugin server, providing the socket address
/// for clients to connect to.
async fn spawn_plugin_server() -> anyhow::Result<()> {
    let listener = LocalSocketListener::bind("/tmp/starship.sock")
        .await?
        .incoming()
        .filter_map(|r| async { r.ok() })
        .map(|r| Transport::from((r.compat(), Bincode::default())));

    let plugin_server = listener
        .map(BaseChannel::with_defaults)
        .execute(PluginServer.serve());
    tokio::spawn(plugin_server);

    Ok(())
}
