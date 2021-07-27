use std::path::PathBuf;

pub use process::*;

mod process;

/// The definition of the common API contract between the server and clients
/// Replaces the need for any Protobuf DSL, or anything like that
#[tarpc::service]
pub trait PluginService {
    async fn current_dir() -> PathBuf;
    async fn output(output: String);
}
