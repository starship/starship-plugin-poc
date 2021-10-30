use service::PluginClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let plugin = PluginClient::try_init().await?;
    let context = tarpc::context::current();

    let dir = plugin.current_dir(context).await?;
    let dir = dir.as_os_str().to_string_lossy().to_string();

    plugin.output(context, dir).await?;

    Ok(())
}
