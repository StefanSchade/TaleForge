use crate::domain::test_data::TEST_DATA;
use crate::port::dto::{MovePlayerInput, MovePlayerOutput};

pub trait MovePlayer {
    fn move_player(&self, input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str>;
}

pub struct MovePlayerUseCase;

impl MovePlayer for MovePlayerUseCase {
    fn move_player(&self, input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str> {
        let mut test_data = TEST_DATA.lock().unwrap(); // Lock the mutex to access the test data
        let player_state = &mut test_data.player_state; // Access the player state

        // Assuming you have a method to determine the new location based on the direction,
        // and the current location of the player, you would call it here.
        // This is a simplified placeholder for the actual game logic.
        let new_location_id = match input.direction.as_str() {
            "north" => Some(2), // Example: Move to location with ID 2 if moving north
            _ => None,
        };

        match new_location_id {
            Some(id) => {
                player_state.current_location_id = id; // Update the player's current location
                // Fetch the new location's details from your game's state or configuration
                // Placeholder response
                Ok(MovePlayerOutput {
                    room_number: id,
                    title: "New Location Title".to_string(),
                    description: "You've moved to a new location.".to_string(),
                    image_url: None, // Assuming an optional field for locations with associated images
                })
            },
            None => Err("Invalid move or direction"),
        }
    }
}
