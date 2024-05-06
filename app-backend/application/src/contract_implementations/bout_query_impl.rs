use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crosscutting::error_management::error::Error;
use crosscutting::error_management::standard_errors::{DATABASE_ACCESS_ERROR, NO_ENTRY_FOUND, UNEXPECTED_ERROR};
use domain_contract::contracts::bout_query::BoutQuery;
use domain_pure::model::bout::Bout;
use port::repositories::bout_repository::BoutRepository;

use crate::dto_domain_mapping::bout_mapper::bout_map_dto_to_domain;

#[derive(Clone, Debug)]
pub struct BoutQueryImpl {
    bout_repository: Arc<dyn BoutRepository>,
}

impl BoutQueryImpl {
    pub fn new(bout_repository: Arc<dyn BoutRepository>) -> Self {
        BoutQueryImpl { bout_repository }
    }
}

impl BoutQuery for BoutQueryImpl {
    fn get_bout_by_id(&self, bout_id: i64) -> Pin<Box<dyn Future<Output=Result<Bout, Error>> + Send + 'static>> {
        let repo = self.bout_repository.clone();
        Box::pin(async move {
            match repo.get_bout_by_id(bout_id).await {
                Ok(Some(bout_dto)) => bout_map_dto_to_domain(bout_dto)
                    .map_err(|err| UNEXPECTED_ERROR.instantiate(vec![err, "Failed to map BoutDTO to Bout".to_string()])),
                Ok(None) => Err(NO_ENTRY_FOUND.instantiate(vec!["bout".to_string(), format!("BoutID={}", bout_id)])),
                Err(e) => Err(DATABASE_ACCESS_ERROR.instantiate(vec![e.message]))
            }
        })
    }
}