#[fp_export_impl(protocol)]
fn metadata() -> Metadata {
    Metadata {
        name: "directory".to_string(),
        description: "Display the current active directory".to_string(),
    }
}

#[fp_export_impl(protocol)]
async fn output() -> String {
    "~".to_string()
}
