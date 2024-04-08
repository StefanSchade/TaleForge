use std::sync::Arc;
use application::contract_implementations::location_query_impl::LocationQueryImpl;
use application::contract_implementations::passage_query_impl::PassageQueryImpl;
use domain_contract::contracts::location_queries::navigation::LocationQueries;
use domain_contract::contracts::passage_queries::navigation::PassageQueries;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use crate::service_container::domain_story_container::DomainStoryContainer;
use crate::service_container::repository_container::RepositoryContainer;
#[derive(Clone)]
pub struct QueryContainer {
    location_queries: Arc<dyn LocationQueries>,
    passage_queries: Arc<dyn PassageQueries>,
}

impl QueryContainer {
    pub fn new(repo_container: &RepositoryContainer) -> Self {
        Self {
            location_queries: Arc::new(LocationQueryImpl::new(repo_container.location())),
            passage_queries: Arc::new(PassageQueryImpl::new(repo_container.passage())),
        }
    }

    pub fn location(&self) -> Arc<dyn LocationQueries> {
        self.location_queries.clone()
    }

    pub fn passage(&self) -> Arc<dyn PassageQueries> {
        self.passage_queries.clone()
    }


}