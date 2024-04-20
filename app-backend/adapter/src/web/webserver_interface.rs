use std::sync::Arc;
use port::service_container::service_container::ServiceContainer;

//#[async_trait]
pub trait WebServer {
    async fn start_server(&self) -> Result<(), std::io::Error>;
    fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send;
}
