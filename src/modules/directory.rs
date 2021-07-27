use std::error::Error;

use tarpc::context;
use tarpc::serde_transport as transport;
use tokio_serde::formats::Bincode;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use starship_rpc_plugin::{PluginServiceClient, Process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let process = Process::new();
    let codec_builder = LengthDelimitedCodec::builder();
    let transport = transport::new(codec_builder.new_framed(process), Bincode::default());
    let client = PluginServiceClient::new(Default::default(), transport).spawn();

    let dir = client.current_dir(context::current()).await?;
    let output = dir.to_string_lossy().to_string();
    client.output(context::current(), output).await?;

    println!("Done!");
    Ok(())
}
