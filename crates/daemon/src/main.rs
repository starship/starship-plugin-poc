use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use once_cell::sync::Lazy;
use plugin_runtime::Runtime;
use shellexpand::tilde;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

mod plugin_runtime;

pub static CACHE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.cache/starship").into_owned()));
pub static CONFIG_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.config/starship").into_owned()));
pub static DATA_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.local/share/starship").into_owned()));

pub static SOCKET_PATH: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("starship.sock"));

fn _init_directories() -> Result<()> {
    if !DATA_DIR.exists() {
        fs::create_dir_all(DATA_DIR.as_path())?;
    }

    if !CONFIG_DIR.exists() {
        fs::create_dir_all(CONFIG_DIR.as_path())?;
    }

    if !CACHE_DIR.exists() {
        fs::create_dir_all(CACHE_DIR.as_path())?;
    }

    Ok(())
}

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

            rt.init().unwrap();

            let dir = rt.output().await.unwrap();
            socket.write_all(dir.as_bytes()).await.unwrap();
        });
    }
}
