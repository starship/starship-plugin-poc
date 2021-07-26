use std::error::Error;

use tarpc::serde_transport as transport;
use tarpc::server::{BaseChannel, Channel};
use tokio_serde::formats::Bincode;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use plugin::{ChildProcess, PluginService, Service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let codec_builder = LengthDelimitedCodec::builder();
    let child_process = ChildProcess::new();
    
    let framed = codec_builder.new_framed(child_process);
    let transport = transport::new(framed, Bincode::default());
    let server = BaseChannel::with_defaults(transport);

    tokio::spawn(server.execute(Service.serve())).await?;

    Ok(())
}
