use fp_bindgen::prelude::Serializable;

#[derive(Serializable)]
pub struct PluginMetadata {
    pub name: String,
    pub description: String,
}
