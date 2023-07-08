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
    for bindings_type in [
        BindingsType::RustPlugin(
            RustPluginConfig::builder()
                .name("starship_bindings")
                .description("Bindings for the Starship plugin protocol")
                .version("0.1.0")
                .license("ISC")
                .dependency(
                    "fp-bindgen-support",
                    CargoDependency::with_version_and_features(
                        "3.0.0",
                        BTreeSet::from(["async", "guest"]),
                    ),
                )
                .build(),
        ),
        BindingsType::RustWasmer2Runtime,
    ] {
        let output_path = format!("bindings/{bindings_type}");

        fp_bindgen!(BindingConfig {
            bindings_type,
            path: &output_path
        });
        println!("Generated bindings written to `{output_path}/`.");
    }
}
