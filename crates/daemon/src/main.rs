use anyhow::Result;

mod plugin_runtime;
use plugin_runtime::bindings::Runtime;

const WASM_BYTES: &[u8] = include_bytes!("../../../target/wasm32-unknown-unknown/debug/starship_plugin_directory.wasm");

fn main() -> Result<()> {
    let rt: Runtime = Runtime::new(WASM_BYTES)?;

    let dir = rt.output()?;
    println!("{}", dir);

    Ok(())
}
