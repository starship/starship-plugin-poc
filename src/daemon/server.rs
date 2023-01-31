use std::{env, path::PathBuf};

use service::Plugin;
use tarpc::context::Context;

#[derive(Clone)]
pub(crate) struct PluginServer;

#[tarpc::server]
impl Plugin for PluginServer {
    async fn current_dir(self, _: Context) -> PathBuf {
        env::current_dir().expect("could not retreive current dir")
    }
    async fn output(self, _: Context, output: String) {
        println!("{}", output);
    }
}
