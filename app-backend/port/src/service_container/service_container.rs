use std::sync::Arc;
use crate::port_services::domain_story_move_player2::MovePlayerDomainStory2;

#[derive(Clone, Debug)]
pub struct ServiceContainer {
    move_player: Arc<dyn MovePlayerDomainStory2>,
}

impl ServiceContainer {
    pub fn new(
        move_player: Arc<dyn MovePlayerDomainStory2>,
    ) -> Self {
        Self {
            move_player: move_player
        }
    }
    pub fn move_player(&self) -> Arc<dyn MovePlayerDomainStory2> {
        self.move_player.clone()
    }
}