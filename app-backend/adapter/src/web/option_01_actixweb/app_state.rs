use std::sync::Arc;
use port::port_services::domain_story_move_player2::MovePlayerDomainStory2;

use port::port_services::domain_story_move_player::MovePlayerDomainStory;

#[derive(Clone)]
pub struct AppState {
    pub move_player_domain_story: Arc<dyn MovePlayerDomainStory2>,
}

impl AppState {
    pub fn new(
        move_player_domain_story: Arc<dyn MovePlayerDomainStory2>,
    ) -> Self {
        AppState {
            move_player_domain_story,
        }
    }
}