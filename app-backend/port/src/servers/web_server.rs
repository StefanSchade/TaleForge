use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait WebServer: Debug {
    fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>>;
}