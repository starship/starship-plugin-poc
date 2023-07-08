use anyhow::Result;
use bindings::Runtime;

mod bindings;

#[tokio::main]
async fn main() -> Result<()> {
    let wasm_bytes = std::fs::read("./plugins/directory/target/wasm32-unknown-unknown/debug/starship_plugin_directory.wasm")?;
    let rt = Runtime::new(wasm_bytes)?;

    let dir = rt.output().await?;
    println!("Directory: {}", dir);

    Ok(())
}
