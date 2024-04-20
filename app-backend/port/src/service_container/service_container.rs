use std::sync::Arc;
use crate::domain_stories::move_player::MovePlayerDomainStory;
use crate::service_container::inbound_ports::InboundPorts;
use crate::service_container::outbound_ports::OutboundPorts;
use crate::repositories::location_repository::LocationRepository;
use crate::repositories::passage_repository::PassageRepository;
use crate::repositories::player_state_repository::PlayerStateRepository;

#[derive(Clone)]
pub struct ServiceContainer {
    repo_container: OutboundPorts,
    domain_story_container: InboundPorts,
}

impl ServiceContainer {
    pub fn new(
        location_repo: Arc<dyn LocationRepository>,
        passage_repo: Arc<dyn PassageRepository>,
        player_state_repo: Arc<dyn PlayerStateRepository>,
        move_player_ds: Arc<dyn MovePlayerDomainStory>
    ) -> Self {
        let repo_container = OutboundPorts::new(location_repo, passage_repo, player_state_repo);
        let domain_story_container = InboundPorts::new(move_player_ds);
        Self {
            repo_container,
            domain_story_container,
        }
    }

    pub fn repo(&self) -> OutboundPorts {
        self.repo_container.clone()
    }

    pub fn domain_story(&self) -> InboundPorts {
        self.domain_story_container.clone()
    }
}