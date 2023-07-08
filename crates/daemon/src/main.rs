use anyhow::Result;
use bindings::Runtime;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

mod bindings;

#[tokio::main]
async fn main() -> Result<()> {
    let wasm_bytes = std::fs::read(
        "./plugins/directory/target/wasm32-unknown-unknown/debug/starship_plugin_directory.wasm",
    )?;
    let rt = Runtime::new(wasm_bytes)?;
    println!("Loaded plugins.");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on port 8080.");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let rt = rt.clone();
        tokio::spawn(async move {
            println!("New connection.");
            let dir = rt.output().await.unwrap();
            socket.write_all(dir.as_bytes()).await.unwrap();
        });
    }
}
