use std::sync::Arc;

use crate::port_services::domain_story_move_player::MovePlayerDomainStory;

#[derive(Clone, Debug)]
pub struct ServiceContainer {
    move_player: Arc<dyn MovePlayerDomainStory>,
}

impl ServiceContainer {
    pub fn new(
        move_player: Arc<dyn MovePlayerDomainStory>,
    ) -> Self {
        Self {
            move_player: move_player
        }
    }
    pub fn move_player(&self) -> Arc<dyn MovePlayerDomainStory> {
        self.move_player.clone()
    }
}