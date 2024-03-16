
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[builder(default)]
    pub image_url: Option<String>,
}
