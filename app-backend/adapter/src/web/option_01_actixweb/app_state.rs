use std::sync::Arc;

use port::port_services::domain_story_move_player::MovePlayerDomainStory;

#[derive(Clone)]
pub struct AppState {
    pub move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl AppState {
    pub fn new(
        move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
    ) -> Self {
        AppState {
            move_player_domain_story,
        }
    }
}