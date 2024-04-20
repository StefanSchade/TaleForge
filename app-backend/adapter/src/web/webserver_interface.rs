use std::sync::Arc;
use actix_web::web;
use async_trait::async_trait;
use port::service_container::service_container::ServiceContainer;

#[async_trait]
pub trait WebServer {
    async fn start_server(&self) -> std::io::Result<()>;
    fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send;
}
