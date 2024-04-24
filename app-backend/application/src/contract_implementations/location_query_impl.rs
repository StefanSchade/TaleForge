use std::future::Future;
use std::pin::Pin;
use domain_pure::model::location::Location;
use port::repositories::location_repository::LocationRepository;
use std::sync::Arc;
use crosscutting::error_management::error::Error;
use domain_contract::contracts::location_query::navigation::LocationQueries;
use crate::dto_domain_mapping::location_mapper::location_map_dto_to_domain;

#[derive(Clone, Debug)]
pub struct LocationQueryImpl {
    location_repository: Arc<dyn LocationRepository>,
}

impl LocationQueryImpl {
    pub fn new(location_repository: Arc<dyn LocationRepository>) -> Self {
        LocationQueryImpl { location_repository }
    }
}

impl LocationQueries for LocationQueryImpl {
    fn get_location_by_aggregate_id(
        &self,
        location_aggregate_id: i32
    ) -> Pin<Box<dyn Future<Output = Result<Option<Location>, Error>> + Send>> {

        let location_repository = self.location_repository.clone();

        Box::pin(async move {
            match location_repository.get_location_by_id(location_aggregate_id).await {
                Ok(Some(location_dto)) => {
                    let location = location_map_dto_to_domain(location_dto);
                    Ok(Some(location))
                },
                Ok(None) => Ok(None),
                Err(e) => Err(e)
            }
        })
    }
}
