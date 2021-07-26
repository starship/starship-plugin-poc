use std::error::Error;

use tarpc::context::{self, Context};
use tarpc::serde_transport as transport;
use tarpc::server::{BaseChannel, Channel};
use tokio_serde::formats::Bincode;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use plugin::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let process = Process::new();

    let codec_builder = LengthDelimitedCodec::builder();
    let transport = transport::new(codec_builder.new_framed(process), Bincode::default());

    let client = PluginServiceClient::new(Default::default(), transport).spawn();
    
    let output = client.hello(context::current(), "Server".to_string()).await?;
    client.output(context::current(), output).await?;

    Ok(())
}
