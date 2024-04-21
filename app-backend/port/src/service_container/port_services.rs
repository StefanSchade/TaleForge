use std::sync::Arc;

use crate::port_services::domain_story_move_player::MovePlayerDomainStory;

#[derive(Clone)]
pub struct PortServices {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl PortServices {
    pub fn new(
        move_player: Arc<dyn MovePlayerDomainStory>,
    ) -> Self {
        Self {
            move_player_domain_story: move_player
        }
    }

    pub fn move_player(&self) -> Arc<dyn MovePlayerDomainStory> {
        self.move_player_domain_story.clone()
    }
}