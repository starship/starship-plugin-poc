use starship_plugin::*;

#[fp_export_impl(starship_plugin)]
fn metadata() -> Metadata {
    Metadata {
        name: "directory".to_string(),
        description: "Display the current active directory".to_string(),
    }
}

#[fp_export_impl(starship_plugin)]
fn output() -> String {
    current_dir();
    "test".to_string()
}
