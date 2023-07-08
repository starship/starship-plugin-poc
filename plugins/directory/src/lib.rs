use starship_bindings::*;

#[fp_export_impl(starship_bindings)]
fn version() -> [u8; 3] {
    [0, 1, 0]
}

#[fp_export_impl(starship_bindings)]
fn metadata() -> PluginMetadata {
    PluginMetadata {
        name: "directory".to_string(),
        description: "The current working directory".to_string(),
    }
}

#[fp_export_impl(starship_bindings)]
async fn output() -> String {
    current_dir()
}
