use futures::StreamExt;
use service::{init_tracing, Plugin};
use std::process::{Command, Stdio};
use tarpc::{
    context,
    serde_transport::tcp,
    server::{BaseChannel, Incoming},
};
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
    init_tracing("starship_plugin_server")?;

    let plugin_list = vec!["./target/debug/client"];

    let listener = tcp::listen("localhost:0", Bincode::default)
        .await?
        .filter_map(|r| async { r.ok() });
    let addr = listener.get_ref().local_addr();
    let server = listener
        .map(BaseChannel::with_defaults)
        .take(plugin_list.len())
        .execute(PluginServer.serve());
    tokio::spawn(server);

    for plugin in plugin_list {
        Command::new(plugin)
            .arg(addr.port().to_string())
            .stdin(Stdio::null())
            .spawn()?
            .wait()?;
    }

    Ok(())
}
