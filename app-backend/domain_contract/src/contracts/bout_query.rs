use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use crosscutting::error_management::error::Error;
use domain_pure::model::bout::Bout;

pub trait BoutQuery: Send + Sync + Debug {
    fn get_bout_by_id(&self, bout_id: u64) -> Pin<Box<dyn Future<Output=Result<Bout, Error>> + Send + 'static>>;
}