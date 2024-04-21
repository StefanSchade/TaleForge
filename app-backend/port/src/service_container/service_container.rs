use std::sync::Arc;
use crate::domain_stories::move_player::MovePlayerDomainStory;
use crate::service_container::port_services::PortServices;
use crate::service_container::outbound_adapters::OutboundAdapters;
use crate::repositories::location_repository::LocationRepository;
use crate::repositories::passage_repository::PassageRepository;
use crate::repositories::player_state_repository::PlayerStateRepository;

#[derive(Clone)]
pub struct ServiceContainer {
    outbound_ports: Arc<OutboundAdapters>,
    inbound_ports: Arc<PortServices>,
}

impl ServiceContainer {
    pub fn new(
        location_repo: Arc<dyn LocationRepository>,
        passage_repo: Arc<dyn PassageRepository>,
        player_state_repo: Arc<dyn PlayerStateRepository>,
        move_player_ds: Arc<dyn MovePlayerDomainStory>
    ) -> Self {
        let repo_container = OutboundAdapters::new(location_repo, passage_repo, player_state_repo);
        let domain_story_container = PortServices::new(move_player_ds);
        Self {
            outbound_ports: Arc::new(repo_container),
            inbound_ports: Arc::new(domain_story_container),
        }
    }

    pub fn outbound(&self) -> Arc<OutboundAdapters> {
        self.outbound_ports.clone()
    }

    pub fn inbound(&self) -> Arc<PortServices> {
        self.inbound_ports.clone()
    }
}