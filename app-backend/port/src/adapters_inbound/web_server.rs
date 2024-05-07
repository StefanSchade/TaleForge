use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait WebServer: Debug {
    fn start_server(
        &self
    ) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>>;
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub use_https: bool,
    pub ssl_key: Option<String>,
    pub ssl_cert: Option<String>,
}

impl ServerConfig {
    pub fn new
    (
        address: String,
        use_https: bool,
        ssl_key: Option<String>,
        ssl_cert:
        Option<String>,
    ) -> Self {
        ServerConfig {
            address,
            use_https,
            ssl_key,
            ssl_cert,
        }
    }
}
