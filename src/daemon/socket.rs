use std::future;

use anyhow::Result;
use futures::StreamExt;
use log::info;
use service::{Plugin, SOCKET_ADDR};
use tarpc::{
    serde_transport::tcp,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Bincode,
};

use crate::server::PluginServer;

pub(crate) async fn accept_incoming() -> Result<()> {
    // TODO: Use Unix Domain Sockets over TCP if possible.

    let listener = tcp::listen(*SOCKET_ADDR, Bincode::default).await?;
    info!("Listening on {}", *SOCKET_ADDR);

    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(BaseChannel::with_defaults)
        .map(|channel| channel.execute(PluginServer.serve()))
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
