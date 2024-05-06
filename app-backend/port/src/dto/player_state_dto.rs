use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStateDTO {
    pub player_id: i64,
    pub bout_id: i64,
    pub current_location_id: i64,
}