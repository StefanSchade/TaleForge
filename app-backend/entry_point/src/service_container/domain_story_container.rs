use std::sync::Arc;
use application::domain_story_impl::move_player_impl::MovePlayerDomainStoryImpl;
use domain_contract::contracts::location_queries::navigation::LocationQueries;
use domain_contract::services::navigation_services::NavigationService;
use port::domain_stories::move_player::MovePlayerDomainStory;
use crate::service_container::domain_service_container::DomainServiceContainer;
use crate::service_container::query_container::QueryContainer;
use crate::service_container::repository_container::RepositoryContainer;

#[derive(Clone)]
pub struct DomainStoryContainer {
    move_player: Arc<dyn MovePlayerDomainStory>,
}


impl DomainStoryContainer {
    pub fn new(repo_container: &RepositoryContainer, domain_service_container: &DomainServiceContainer) -> Self {
        Self {
            move_player: Arc::new(MovePlayerDomainStoryImpl::new(
                repo_container.location(),
                repo_container.passage(),
                repo_container.player_state(),
                domain_service_container.navigation()))
        }
    }

    pub fn move_player(&self) -> Arc<dyn MovePlayerDomainStory> {
        self.move_player.clone()
    }

}