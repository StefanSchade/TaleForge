use crate::domain::aggregates::{location::Location, passage::Passage};
use crate::domain::aggregates::player_state::PlayerState;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Definition of the TestData struct
pub struct TestData {
    pub player_state: PlayerState,
    pub test_world: (Vec<Location>, Vec<Passage>, PlayerState),
    // You can add other fields here as needed
}

pub fn setup_test_domain() -> (Vec<Location>, Vec<Passage>, PlayerState) {
    let room1 = Location {
        id: 1,
        title: "Entrance Hall".to_string(),
        description: "You are standing in a grand entrance hall. The air is cold and still.".to_string(),
        image_url: None,
        exits: vec![2], // Assuming room 2 is directly accessible from room 1
    };

    let room2 = Location {
        id: 2,
        title: "Great Library".to_string(),
        description: "Surrounded by towering shelves of books, the scent of old paper fills the air.".to_string(),
        image_url: None,
        exits: vec![1], // Assuming room 1 is directly accessible from room 2
    };

    let passage_between_rooms = Passage {
        id: 1,
        from_location_id: room1.id,
        to_location_id: room2.id,
        description: "A narrow, creaky passage that connects the entrance hall to the great library.".to_string(),
    };

    let player_state = PlayerState::new(room1.id); // Initializing the player in room 1

    let test_world = (vec![room1, room2], vec![passage_between_rooms], player_state);

    test_world
}

impl TestData {
    pub fn new() -> Self {
        let (test_, _, player_state) = setup_test_domain();
        TestData {
            player_state,
            test_world: setup_test_domain(),
            // Initialize other fields as needed
        }
    }
}

// Using lazy_static to define a globally accessible TestData instance wrapped in a Mutex
lazy_static! {
    pub static ref TEST_DATA: Mutex<TestData> = Mutex::new(TestData::new());
}
