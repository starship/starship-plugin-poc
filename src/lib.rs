use anyhow::anyhow;
use std::{env, net::Ipv6Addr, path::PathBuf};
use tarpc::{client, serde_transport::tcp};
use tokio_serde::formats::Bincode;

use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

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
        let port = env::args()
            .nth(1)
            .ok_or_else(|| anyhow!("Port required."))?;
        let addr = (Ipv6Addr::LOCALHOST, port.parse()?);

        let transport = tcp::connect(addr, Bincode::default).await?;
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
