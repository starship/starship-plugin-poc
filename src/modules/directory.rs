use service::PluginClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the plugin RPC with the server that spawned it
    let plugin = PluginClient::init();
    let context = tarpc::context::current();

    // Perform an RPC call requesting the current directory
    let dir = plugin.current_dir(context).await?;
    let dir = dir.as_os_str().to_string_lossy().to_string();

    // Perform an RPC call providing the module's output
    plugin.output(context, dir).await?;

    // Printing something is necessary to end the process
    // Hangs indefinitely without it. ¯\_(ツ)_/¯
    println!("Done!");
    Ok(())
}
