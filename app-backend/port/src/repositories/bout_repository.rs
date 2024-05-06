use std::fmt::Debug;
use futures::future::BoxFuture;
use crosscutting::error_management::error::Error;
use crate::dto::bout_dto::BoutDTO;

pub trait BoutRepository: Send + Sync + Debug {
    fn get_bout_by_id(&self, match_id: i64) -> BoxFuture<'static, Result<Option<BoutDTO>, Error>>;
    fn add_bout(&self, bout: BoutDTO) -> BoxFuture<'static, Result<(), Error>>;
}