use anyhow::Result;

mod plugin_runtime;
use bindings::*;

mod types;
pub use types::*;

const WASM_BYTES: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/debug/guest.wasm");

fn main() -> Result<()> {
    let rt: Runtime = Runtime::new(WASM_BYTES)?;

    let metadata = rt.metadata()?;

    println!("metadata: {:?}", metadata);

    Ok(())
}
