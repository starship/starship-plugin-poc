use std::path::PathBuf;

use starship_plugins_poc::{MergedChildIO, Plugin};
use tarpc::{
    context::Context,
    serde_transport,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Bincode,
};
use tokio_util::codec::LengthDelimitedCodec;

#[derive(Clone)]
struct PluginServer;

// Implement the `PluginService` trait defined in `lib.rs`, forming an
// API contract between the server and client using Rust's strong types.
// The types are (de)serialized with Bincode, so we can use nearly any types.
#[tarpc::server]
impl Plugin for PluginServer {
    /// Retreive the current working directory.
    async fn current_dir(self, _: Context) -> PathBuf {
        std::env::current_dir().expect("could not retreive current dir")
    }

    /// Provide the output for this module.
    async fn output(self, _: Context, output: String) {
        println!("{}", output);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut plugin_handles = Vec::with_capacity(20);

    // We're going to try spawning 20 plugins
    for _ in 1..20 {
        // Spawn child process
        let merged_io = MergedChildIO::new("./target/release/client");

        // Initialize RPC
        let codec_builder = LengthDelimitedCodec::builder();
        let framed = codec_builder.new_framed(merged_io);
        let transport = serde_transport::new(framed, Bincode::default());
        let fut = BaseChannel::with_defaults(transport).execute(PluginServer.serve());

        // Add to list of concurrent handles
        plugin_handles.push(tokio::spawn(fut));
    }

    futures::future::join_all(plugin_handles).await;

    Ok(())
}
