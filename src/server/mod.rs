use std::sync::{Arc, Mutex};
mod server;
mod request;
mod threading;
mod errors;

use request::RequestMethods;

pub struct Handler {
    method: RequestMethods,
    path: String,
    handler: Box<dyn Fn() + Send + Sync>,
}

pub struct Server {
    pub host: String,
    pub port: u16,
    pool_size: usize,
    handlers: Arc<Mutex<Vec<Handler>>>,
}
