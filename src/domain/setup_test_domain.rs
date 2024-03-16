use crate::domain::aggregates::{location::Location, passage::Passage};
use crate::domain::aggregates::player_state::PlayerState;

pub fn setup_test_domain() -> (Vec<Location>, Vec<Passage>, PlayerState) {
    let room1 = Location {
        id: 1,
        title: "Entrance Hall".to_string(),
        description: "You are standing in a grand entrance hall. The air is cold and still.".to_string(),
        image_url: None,

    };

    let room2 = Location {
        id: 2,
        title: "Great Library".to_string(),
        description: "Surrounded by towering shelves of books, the scent of old paper fills the air.".to_string(),
        image_url: None,

    };

    let passage_between_rooms = Passage {
        id: 1,
        from_location_id: room1.id,
        to_location_id: room2.id,
        description: "A narrow, creaky passage that connects the entrance hall to the great library.".to_string(),
    };

    let player_state = PlayerState::new(room1.id); // Initializing the player in room 1

    (vec![room1, room2], vec![passage_between_rooms], player_state)
}
