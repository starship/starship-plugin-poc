use starship_plugins_poc::PluginClient;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let client = PluginClient::init();
    let context = tarpc::context::current();

    // Perform an RPC call requesting the current directory
    let dir = client.current_dir(context).await?;
    let dir = dir.as_os_str().to_string_lossy().to_string();

    // Perform an RPC call providing the module's output
    client.output(context, dir).await?;

    Ok(())
}
