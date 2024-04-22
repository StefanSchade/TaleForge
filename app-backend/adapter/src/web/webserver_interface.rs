use std::future::Future;
use std::sync::Arc;
use port::service_container::service_container::ServiceContainer;

//#[async_trait]
pub trait WebServer: Send + Sync {
    fn start_server(&self) -> impl Future<Output = Result<(), std::io::Error>> + Send;
    fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send;
}
