use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationDTO {
    pub id: i64,
    pub game_id: i64,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
}