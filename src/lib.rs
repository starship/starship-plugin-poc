use std::path::PathBuf;

pub use process::*;

mod process;

#[tarpc::service]
pub trait PluginService {
    async fn current_dir() -> PathBuf;
    async fn output(output: String);
}
