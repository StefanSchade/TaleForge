use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crosscutting::error_management::error::Error;
use port::repositories::bout_repository::BoutRepository;
use domain_contract::contracts::bout_query::BoutQuery;
use domain_pure::model::bout::Bout;

#[derive(Clone, Debug)]
pub struct BoutQueryImpl {
    bout_repository: Arc<dyn BoutRepository>
}

impl BoutQueryImpl {
    pub fn new(bout_repository: Arc<dyn BoutRepository>) -> Self {
        BoutQueryImpl {bout_repository}
    }
}

impl BoutQuery for BoutQueryImpl {
    fn get_bout_by_id(&self, bout_id: u64) -> Pin<Box<dyn Future<Output=Result<Bout, Error>> + Send + 'static>> {
        todo!()
    }
}