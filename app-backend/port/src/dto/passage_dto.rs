use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassageDTO {
    pub id: u64,
    pub from_location_id: u64,
    pub to_location_id: u64,
    pub description: String,
    pub direction: String,
    pub narration: String,
}
