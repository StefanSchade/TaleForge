use std::sync::Arc;

use crate::domain_stories::move_player::MovePlayerDomainStory;

#[derive(Clone)]
pub struct DomainStoryContainer {
    move_player: Arc<dyn MovePlayerDomainStory>,
}

impl DomainStoryContainer {
    pub fn new(
        move_player: Arc<dyn MovePlayerDomainStory>,
    ) -> Self {
        Self {
            move_player
        }
    }

    pub fn move_player(&self) -> Arc<dyn MovePlayerDomainStory> {
        self.move_player.clone()
    }
}