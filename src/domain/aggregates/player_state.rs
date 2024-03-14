pub struct PlayerState {
    pub current_location_id: i32,
}

impl PlayerState {
    pub fn new(start_location_id: i32) -> Self {
        PlayerState {
            current_location_id: start_location_id,
        }
    }

    pub fn move_to(&mut self, new_location_id: i32) -> Result<(), &'static str> {
        // This is a simplified logic. In a real application, you would check if the movement is valid based on the current location's exits and possibly other conditions.
        self.current_location_id = new_location_id;
        Ok(())
    }

}