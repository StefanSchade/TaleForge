use std::sync::Arc;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;
use crate::service_container::domain_service_container::DomainServiceContainer;
use crate::service_container::domain_story_container::DomainStoryContainer;
use crate::service_container::query_container::QueryContainer;
use crate::service_container::repository_container::RepositoryContainer;

#[derive(Clone)]
pub struct ServiceContainer {
    repo_container: RepositoryContainer,
    query_container: QueryContainer,
    domain_story_container: DomainStoryContainer,
}

impl ServiceContainer {
    pub fn new() -> Self {
        let repo_container = RepositoryContainer::new();
        let query_container = QueryContainer::new(&repo_container);
        let domain_service_container=  DomainServiceContainer::new(query_container.clone());
        let domain_story_container = DomainStoryContainer::new(
            &repo_container,
            &domain_service_container);
        Self {
            repo_container,
            query_container,
            domain_story_container,
        }
    }

    pub fn repo(&self) -> RepositoryContainer {
        self.repo_container.clone()
    }

    pub fn query(&self) -> QueryContainer {
        self.query_container.clone()
    }

    pub fn domain_story(&self) -> DomainStoryContainer {
        self.domain_story_container.clone()
    }

}