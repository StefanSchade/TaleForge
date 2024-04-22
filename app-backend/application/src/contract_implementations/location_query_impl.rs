use std::future::Future;
use std::pin::Pin;
use domain_pure::model::location::{Location, LocationBuilder};
use port::repositories::location_repository::LocationRepository;
use std::sync::Arc;
use crosscutting::error_management::error::Error;
use domain_contract::contracts::location_query::navigation::LocationQueries;

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
        Box::pin(async move {
            match self.location_repository.get_location_by_id(location_aggregate_id).await {
                Ok(Some(location_dto)) => {
                    let location = LocationBuilder::from_dto(&location_dto).build(); // Assuming a builder pattern for conversion
                    Ok(Some(location))
                },
                Ok(None) => Ok(None),
                Err(e) => Err(e)
            }
        })
    }
}
