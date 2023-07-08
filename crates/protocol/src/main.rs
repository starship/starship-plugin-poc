use std::collections::BTreeSet;

use fp_bindgen::{prelude::*, types::CargoDependency};

mod types;
use types::*;

fp_import! {
    // TODO: Replace with PathBuf
    fn current_dir() -> String;
}

fp_export! {
    fn version() -> [u8; 3];
    fn metadata() -> PluginMetadata;
    async fn output() -> String;
}

fn main() {
    // Generate bindings for plugin authors
    let plugin_config = RustPluginConfig::builder()
        .name("starship_plugin")
        .description("Bindings used to create a starship plugin")
        .version("0.1.0")
        // `fp-bindgen` automatically adds `fp-bindgen-support` as a dependency
        // but doesn't include the `async` feature when needed.
        .dependency(
            "fp-bindgen-support",
            CargoDependency::with_version_and_features("3.0.0", BTreeSet::from(["async", "guest"])),
        )
        .build();
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustPlugin(plugin_config),
        path: "../starship_plugin"
    });

    // Generate bindings for plugin authors
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustWasmer2Runtime,
        path: "../daemon/src/plugin_runtime",
    });
}
