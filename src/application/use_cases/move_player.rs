use actix_web::http::header::LOCATION;
use serde::__private::ser::serialize_tagged_newtype;
use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::player_state;
use crate::domain::test_data::{TEST_DATA, TestData};
use crate::domain::navigation_services;
use crate::domain::navigation_services::NavigationService;
use crate::port::dto::{MovePlayerInput, MovePlayerOutput};

pub trait MovePlayer {
    fn move_player(&self, input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str>;
}

pub struct MovePlayerUseCase;

impl MovePlayer for MovePlayerUseCase {
    fn move_player(&self, input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str> {
        let mut test_data = TEST_DATA.lock().unwrap(); // Access global test data
        let player_state = &mut test_data.player_state;

        // Domain logic to update player location based on direction
        //NavigationService.update_location_based_on_direction(input.direction).map_err(|_| "Invalid move or direction")?;

        // Fetch updated location details from the domain model
        player_state.current_location_id();

        // Construct and return the dynamic output

        let new_location = TEST_DATA.into_inner().unwrap().test_world.0.pop();

        Ok(MovePlayerOutput {
            room_number: new_location.unwrap().id,
            title: new_location.unwrap().title.clone(),
            description: new_location.unwrap().description.clone(),
            image_url: new_location.unwrap().image_url.clone(),
        })
    }
}
