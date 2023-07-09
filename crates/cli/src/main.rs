use starship_service::StarshipServiceClient;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::serde_transport::tcp;
use tarpc::tokio_serde::formats::Bincode;
use tarpc::{client, context};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 8000);

    let transport = tcp::connect(&server_addr, Bincode::default);

    let client = StarshipServiceClient::new(client::Config::default(), transport.await?).spawn();

    let prompt = client.prompt(context::current()).await?;
    println!("{}", prompt);

    Ok(())
}
