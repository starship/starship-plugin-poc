use std::error::Error;

use tarpc::context;
use tarpc::serde_transport as transport;
use tokio_serde::formats::Bincode;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use starship_rpc_plugin::{PluginServiceClient, Process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Produce a tarpc transport using stdout and stdin as our Sink and Stream
    let process = Process::new();
    let codec_builder = LengthDelimitedCodec::builder();
    let transport = transport::new(codec_builder.new_framed(process), Bincode::default());
    let client = PluginServiceClient::new(Default::default(), transport).spawn();

    // Perform a remote call to get the server's `current_dir`
    let dir = client.current_dir(context::current()).await?;
    let output = dir.to_string_lossy().to_string();
    // Send the output to the server before wrapping up
    client.output(context::current(), output).await?;

    // Printing something is necessary to end the process 
    // Hangs indefinitely without it. ¯\_(ツ)_/¯
    println!("Done!");
    Ok(())
}
