use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStateDTO {
    pub player_id: i32,
    pub current_location_id: i32,
}
