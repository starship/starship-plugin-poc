use anyhow::anyhow;
use service::{init_tracing, PluginClient};
use std::{env::args, net::Ipv6Addr};
use tarpc::{client, context};
use tokio_serde::formats::Bincode;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing("Starship Example Plugin")?;

    let port = args().nth(1).ok_or_else(|| anyhow!("Port required."))?;
    let server_addr = (Ipv6Addr::LOCALHOST, port.parse()?);
    let transport = tarpc::serde_transport::tcp::connect(server_addr, Bincode::default);

    let client = PluginClient::new(client::Config::default(), transport.await?).spawn();
    let output = client
        .hello(context::current(), "matchai".to_string())
        .await?;

    tracing::info!("{:?}", output);

    Ok(())
}
