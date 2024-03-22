#[derive(PartialEq, Debug, Clone)]
pub struct PlayerState {
    pub player_id: i32,
    pub current_location_id: i32,
}

impl PlayerState {
    pub fn new(new_player_id: i32, start_location_id: i32) -> Self {
        PlayerState { player_id: new_player_id, current_location_id: start_location_id }
    }

    pub fn current_location_id(&self) -> i32 {
        self.current_location_id
    }

    pub fn set_current_location_id(&mut self, location_id: i32) {
        self.current_location_id = location_id;
    }
}
