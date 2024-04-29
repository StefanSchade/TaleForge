// use std::sync::Arc;
// use async_trait::async_trait;
// use port::adapters_outbound::adapters_outbound::ServiceContainer;
//
// #[async_trait]
// pub trait WebServer: Send + Sync {
//     async fn start_server(&self) -> Result<(), std::io::Error>;
//     fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send;
// }