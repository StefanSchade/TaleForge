use std::fmt::Debug;

use futures::future::BoxFuture;

use crosscutting::error_management::error::Error;
use domain_pure::model::passage::Passage;

pub trait PassageQueries: Send + Sync + Debug {
    fn find_passage_between_locations(&self, game_id: i64,  from_location_id: i64, to_location_id: i64) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
    fn find_passage_by_location_and_direction(&self, game_id: i64, location_id: i64, direction: &str) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
}