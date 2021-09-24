use std::process::Stdio;

use futures::{future, StreamExt};
use service::{init_tracing, Plugin};
use tarpc::{
    context,
    serde_transport::tcp,
    server::{BaseChannel, Incoming},
};
use tokio::process::Command;
use tokio_serde::formats::Bincode;

#[derive(Clone)]
struct PluginServer;

#[tarpc::server]
impl Plugin for PluginServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        println!("Message from {}", name);
        format!("Hello, {}!", name)
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
    ];

    let listener = tcp::listen("localhost:0", Bincode::default)
        .await?
        .filter_map(|r| async { r.ok() });
    let addr = listener.get_ref().local_addr();
    let plugin_server = listener
        .map(BaseChannel::with_defaults)
        .execute(PluginServer.serve());
    tokio::spawn(plugin_server);

    let server_port = addr.port().to_string();

    let mut plugin_handles = Vec::with_capacity(plugin_list.len());
    for plugin in plugin_list {
        let child = Command::new(plugin)
            .arg(&server_port)
            .stdin(Stdio::null())
            .spawn();

        plugin_handles.push(tokio::spawn(async move { child?.wait().await }));
    }

    future::join_all(plugin_handles).await;

    Ok(())
}
