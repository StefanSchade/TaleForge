use std::sync::Arc;
use port::service_container::service_container::ServiceContainer;

//#[async_trait]
pub trait WebServer {
    fn start_server(&self) -> impl std::future::Future<Output = std::io::Result<()>> + Send;
    fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send;
}
