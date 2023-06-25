use std::{fs, path::PathBuf};

use anyhow::Result;
use extism::{Context, CurrentPlugin, Function, Plugin, UserData, Val, ValType};
use once_cell::sync::Lazy;
use shellexpand::tilde;

pub static CACHE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.cache/starship").into_owned()));
pub static CONFIG_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.config/starship").into_owned()));
pub static DATA_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(tilde("~/.local/share/starship").into_owned()));

pub static SOCKET_PATH: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("starship.sock"));

fn main() -> Result<()> {
    pretty_env_logger::init();

    init_directories()?;

    let wasm = include_bytes!("../../target/wasm32-unknown-unknown/release/plugin.wasm");
    let context = Context::new();

    let mut plugin = Plugin::new(&context, wasm, [current_dir], false)?;
    let data = plugin.call("current_dir", "").unwrap();

    Ok(())
}

fn current_dir(
    _plugin: &mut CurrentPlugin,
    _inputs: &[Val],
    outputs: &mut [Val],
    _user_data: UserData,
) -> Result<()> {
    // outputs[0] = std::env::current_dir().unwrap().to_str().unwrap().into();
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
