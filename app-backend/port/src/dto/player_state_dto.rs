use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStateDTO {
    pub player_id: u64,
    pub bout_id: u64,
    pub current_location_id: u64,
}