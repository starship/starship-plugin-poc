use fp_bindgen::prelude::Serializable;

#[derive(Serializable)]
pub struct Metadata {
    pub name: String,
    pub description: String,
}
