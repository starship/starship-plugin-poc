use service::{MergedProcessIO, PluginClient};
use tarpc::serde_transport;
use tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let merged_io = MergedProcessIO::new();

    let codec_builder = LengthDelimitedCodec::builder();
    let framed = codec_builder.new_framed(merged_io);
    let transport = serde_transport::new(framed, Bincode::default());
    PluginClient::new(Default::default(), transport)
        .spawn()
        .hello(tarpc::context::current(), "matchai".to_string())
        .await?;

    Ok(())
}
