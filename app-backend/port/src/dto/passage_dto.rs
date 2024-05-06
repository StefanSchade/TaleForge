use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassageDTO {
    pub id: i64,
    pub game_id: i64,
    pub from_location_id: i64,
    pub to_location_id: i64,
    pub description: String,
    pub direction: String,
    pub narration: String,
}