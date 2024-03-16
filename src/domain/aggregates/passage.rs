use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Passage {
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub description: String,
}