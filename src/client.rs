use anyhow::anyhow;
use service::{init_tracing, PluginClient};
use std::{env::args, net::Ipv6Addr};
use tarpc::{client, context, serde_transport::tcp};
use tokio_serde::formats::Bincode;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing("Starship Example Plugin")?;

    let port = args().nth(1).ok_or_else(|| anyhow!("Port required."))?;
    let addr = (Ipv6Addr::LOCALHOST, port.parse()?);

    let transport = tcp::connect(addr, Bincode::default).await?;
    let client = PluginClient::new(client::Config::default(), transport).spawn();

    client
        .hello(context::current(), "matchai".to_string())
        .await?;

    client
        .hello(context::current(), "matchai2".to_string())
        .await?;

    Ok(())
}
