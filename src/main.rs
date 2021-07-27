use std::env;
use std::error::Error;
use std::path::PathBuf;

use tarpc::context::Context;
use tarpc::serde_transport as transport;
use tarpc::server::{BaseChannel, Channel};
use tokio_serde::formats::Bincode;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

use starship_rpc_plugin::{ChildProcess, PluginService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let codec_builder = LengthDelimitedCodec::builder();
    let child_process = ChildProcess::new("target/debug/starship-plugin-directory");

    let framed = codec_builder.new_framed(child_process);
    let transport = transport::new(framed, Bincode::default());
    let server = BaseChannel::with_defaults(transport);

    tokio::spawn(server.execute(Service.serve())).await?;

    Ok(())
}

#[derive(Clone)]
pub struct Service;

#[tarpc::server]
impl PluginService for Service {
    async fn current_dir(self, _: Context) -> PathBuf {
        env::current_dir().expect("could not retreive current dir")
    }

    async fn output(self, _: Context, output: String) {
        println!("{}", output);
    }
}
