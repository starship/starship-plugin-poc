use starship_plugin::*;

#[fp_export_impl(starship_plugin)]
fn version() -> [u8; 3] {
    [0, 1, 0]
}

#[fp_export_impl(starship_plugin)]
fn metadata() -> PluginMetadata {
    PluginMetadata {
        name: "directory".to_string(),
        description: "The current working directory".to_string(),
    }
}

#[fp_export_impl(starship_plugin)]
async fn output() -> String {
    current_dir()
}
