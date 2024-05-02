use std::sync::Arc;

use openapi_client::api_adapter::{BusinessAdapter, MovePlayerOpenApiResult};  // Adjust if the path differs
use port::context::RequestContext;
use port::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerDomainStory};

pub struct BusinessAdapterImpl {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl BusinessAdapterImpl {
    pub fn new(move_player_domain_story: Arc<dyn MovePlayerDomainStory>) -> Self {
        BusinessAdapterImpl {  // Corrected constructor return type
            move_player_domain_story
        }
    }
}

impl BusinessAdapter for BusinessAdapterImpl {
    async fn move_player(&self, bout_id: u64, player_id: u64, direction: String) -> Result<MovePlayerOpenApiResult, String> {
        let context = RequestContext::new(bout_id, player_id);
        let input = MovePlayerCommand { direction };

        match self.move_player_domain_story.execute(context, input).await {
            Ok(move_player_result) => {
                Ok(MovePlayerOpenApiResult {
                    player_location: move_player_result.player_location,
                    narration: move_player_result.narration,
                })
            },
            Err(e) => Err(e.to_string()),  // Convert error to string if needed
        }
    }
}
