use std::sync::Arc;

use anyhow::Result;
use service::PluginClient;

#[tokio::main]
async fn main() -> Result<()> {
    let plugin = PluginClient::try_init().await?;
    let plugin = Arc::new(plugin);
    let context = tarpc::context::current();

    // Wait for all of the concurrent calls to finish
    futures::future::join_all((0..50).map(|i| {
        let plugin = plugin.clone();
        tokio::spawn(async move {
            plugin.current_dir(context).await.unwrap();
            println!("Finished {i}");
        })
    }))
    .await;

    println!("Done!");

    Ok(())
}
