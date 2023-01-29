use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::{Context, Result};
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

pub(crate) async fn accept_incoming() -> Result<()> {
    // TODO: Use Unix Domain Sockets over TCP if possible.

    let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 27812);
    info!("Listening on {address}");

    let listener = TcpListener::bind(address)
        .await
        .with_context(|| format!("Failed bind to address: {address}"))?;

    loop {
        // Poll incoming connections.
        let (stream, addr) = match listener.accept().await {
            Ok((stream, addr)) => {
                info!("Accepted connection from {addr}");
                (stream, addr)
            }
            Err(e) => {
                error!("Failed to accept incoming connection: {e}");
                continue;
            }
        };

        tokio::spawn(async move {
            let _result = handle_incoming(stream, addr).await;
        });
    }
}

async fn handle_incoming(stream: TcpStream, addr: SocketAddr) -> Result<()> {
    todo!()
}
