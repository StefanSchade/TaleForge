use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct PlayerState {
    player_id: u64, //aggregate id and foreign kez (once I have established a user)
    current_location_id: u64,
}

impl PlayerState {

    pub fn current_location_id(&self) -> u64 {
        self.current_location_id
    }
    pub fn set_current_location_id(&mut self, location_id: u64) {
        self.current_location_id = location_id;
    }
    pub fn player_id(&self) -> u64 {self.player_id}
}
