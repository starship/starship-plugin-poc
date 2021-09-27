use service::PluginClient;
use tarpc::context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = PluginClient::try_init().await?;

    client
        .hello(context::current(), "matchai".to_string())
        .await?;
    let _dir = client
        .current_dir(context::current())
        .await?
        .map_err(anyhow::Error::msg)?;

    Ok(())
}
