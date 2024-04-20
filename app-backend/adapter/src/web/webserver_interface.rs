use std::sync::Arc;
use actix_web::web;
use port::service_container::service_container::ServiceContainer;

//#[async_trait]
pub trait WebServer {
    async fn start_server(container: web::Data<Arc<ServiceContainer>>) -> std::io::Result<()>;
}
