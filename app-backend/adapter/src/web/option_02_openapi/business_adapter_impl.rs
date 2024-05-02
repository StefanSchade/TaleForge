use std::sync::Arc;

use ::openapi_client::api_adapter::BusinessAdapter;
use port::context::RequestContext;
use port::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerDomainStory};

pub struct BusinessAdapterImpl {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl BusinessAdapterImpl {
    pub fn new(move_player_domain_story: Arc<dyn MovePlayerDomainStory>) -> Self {
        BusinessAdapter {
            move_player_domain_story
        }
    }
}

impl BusinessAdapter for BusinessAdapterImpl {
    async fn move_player(&self, bout_id: u64, player_id: u64, direction: String) -> Result<String, String> {
        let context = RequestContext::new(bout_id, player_id);
        let input = MovePlayerCommand {
            direction
        };
        match self.move_player_domain_story.execute(context, input).await {
            Ok(move_player_result) => {
                // Serialize the MovePlayerResult to JSON
                serde_json::to_string(&move_player_result)
                    .map_err(|e| e.to_string())  // Convert any serialization error into a string
            },
            Err(e) => Err(e),
        }
    }
}