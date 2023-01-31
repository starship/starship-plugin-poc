use std::{fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use shellexpand::tilde;

mod pid;
mod socket;
mod server;

pub static CACHE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.cache/starship").into_owned()));
pub static CONFIG_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.config/starship").into_owned()));
pub static DATA_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.local/share/starship").into_owned()));

pub static SOCKET_PATH: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("starship.sock"));

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    init_directories()?;
    pid::create_pid_file()?;

    socket::accept_incoming().await?;

    Ok(())
}

fn init_directories() -> Result<()> {
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
