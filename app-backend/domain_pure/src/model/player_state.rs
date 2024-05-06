use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct PlayerState {
    player_id: i64,
    bout_id: i64,
    current_location_id: i64,
}

impl PlayerState {
    pub fn current_location_id(&self) -> i64 {
        self.current_location_id
    }
    pub fn set_current_location_id(&mut self, location_id: i64) {
        self.current_location_id = location_id;
    }
    pub fn player_id(&self) -> i64 { self.player_id }
    pub fn bout_id(&self) -> i64 { self.bout_id }
}