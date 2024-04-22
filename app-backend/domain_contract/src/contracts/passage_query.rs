use futures::future::BoxFuture;
use crosscutting::error_management::error::Error;
use domain_pure::model::passage::Passage;

pub trait PassageQueries: Send + Sync {
    fn find_passage_between_locations(&self, from_location_id: i32, to_location_id: i32) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
}