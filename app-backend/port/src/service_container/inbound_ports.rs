use std::sync::Arc;

use crate::domain_stories::move_player::MovePlayerDomainStory;

#[derive(Clone)]
pub struct InboundPorts {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl InboundPorts {
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