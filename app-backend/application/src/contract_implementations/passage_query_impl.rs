use domain_pure::model::passage::Passage;
use crate::dto_domain_mapping::passage_mapper::passage_map_dto_to_domain;
use port::repositories::passage_repository::PassageRepository;
use std::sync::Arc;
use futures::future::BoxFuture;
use crosscutting::error_management::error::Error;
use domain_contract::contracts::passage_query::PassageQueries;

#[derive(Clone, Debug)]
pub struct PassageQueryImpl {
    passage_repository: Arc<dyn PassageRepository>,
}

impl PassageQueryImpl {
    pub fn new(passage_repository: Arc<dyn PassageRepository>) -> Self {
        PassageQueryImpl { passage_repository }
    }
}

impl PassageQueries for PassageQueryImpl {
    fn find_passage_between_locations(&self, from_location_id: u64, to_location_id: u64) -> BoxFuture<'static, Result<Option<Passage>, Error>> {
        let repo = self.passage_repository.clone();
        Box::pin(async move {
            let passage_dto_result = repo.find_by_start_and_end_id(from_location_id, to_location_id).await;
            passage_dto_result.map(|option_dto| option_dto.map(passage_map_dto_to_domain)).map_err(|e| e.into())
        })
    }

    fn find_passage_by_location_and_direction(&self, location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<Passage>, Error>> {
        let repo = self.passage_repository.clone();
        let direction = direction.to_owned();  // Clone to capture in async block
        Box::pin(async move {
            let passage_dto_result = repo.find_passage_by_location_and_direction(location_id, &direction).await;
            passage_dto_result.map(|option_dto| option_dto.map(passage_map_dto_to_domain)).map_err(|e| e.into())
        })
    }
}
