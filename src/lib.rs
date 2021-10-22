use interprocess::nonblocking::local_socket::LocalSocketStream;
use std::path::PathBuf;
use tarpc::{client, serde_transport::Transport};
use tokio_serde::formats::Bincode;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing_subscriber::{
    fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait Plugin {
    async fn hello(name: String) -> String;
    async fn current_dir() -> Result<PathBuf, String>;
}

impl PluginClient {
    /// Initialize a plugin client:
    /// - Initialize tracing for the plugin
    /// - Connect to the plugin server by the provided port
    /// - Configure the TCP transport
    pub async fn try_init() -> anyhow::Result<Self> {
        init_tracing()?;
        let stream = LocalSocketStream::connect("/tmp/starship.sock")
            .await
            .unwrap();
        let stream = stream.compat();
        let transport = Transport::from((stream, Bincode::default()));

        let plugin_client = PluginClient::new(client::Config::default(), transport).spawn();
        Ok(plugin_client)
    }
}

pub fn init_tracing() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .try_init()?;

    Ok(())
}
