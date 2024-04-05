use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PassageDTO {
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub description: String,
    pub direction: String,
    pub narration: String,
}
