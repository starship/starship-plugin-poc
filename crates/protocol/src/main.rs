use std::collections::BTreeSet;

use fp_bindgen::{prelude::*, types::CargoDependency};

mod types;
pub use types::*;

// Daemon functions available to the plugin
fp_import! {
    fn current_dir();
}

// Plugin functions available to the daemon
fp_export! {
    fn metadata() -> Metadata;
    fn output() -> String;
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
