#[derive(Debug, Clone)]
pub struct PlayerState {
    current_location_id: i32,
}

impl PlayerState {
    pub fn new(start_location_id: i32) -> Self {
        PlayerState { current_location_id: start_location_id }
    }

    // Rust "getter" equivalent for current_location_id
    pub fn current_location_id(&self) -> i32 {
        self.current_location_id
    }

    // Optionally, if you need to change the location from outside this struct
    pub fn set_current_location_id(&mut self, location_id: i32) {
        self.current_location_id = location_id;
    }
}
