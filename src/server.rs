use std::process::Stdio;

use service::{MergedChildIO, Plugin, PluginClient};
use tarpc::{
    context, serde_transport,
    server::{BaseChannel, Channel},
};
use tokio::process::Command;
use tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

/// This is the type that implements the generated World trait. It is the business logic
/// and is used to start the server.
#[derive(Clone)]
struct PluginServer;

#[tarpc::server]
impl Plugin for PluginServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {}!", name)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let codec_builder = LengthDelimitedCodec::builder();
    tokio::spawn(async move {
        let command = Command::new("./target/debug/client")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("child spawned correctly");

        let merged_io = MergedChildIO::new(command);
        let framed = codec_builder.new_framed(merged_io);
        let transport = serde_transport::new(framed, Bincode::default());
        let fut = BaseChannel::with_defaults(transport).execute(PluginServer.serve());
        tokio::spawn(fut);
    });

    Ok(())
}
