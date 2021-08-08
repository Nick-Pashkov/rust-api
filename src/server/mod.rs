use std::sync::{Arc, Mutex};
mod server;
mod request;
mod response;
mod threading;
mod errors;

use request::RequestMethods;
use request::Request;
use response::Response;

pub struct Handler {
    method: RequestMethods,
    path: String,
    handler: Box<dyn Fn(&Request, &mut Response) + Send + Sync>,
}

pub struct Server {
    pub host: String,
    pub port: u16,
    pool_size: usize,
    handlers: Arc<Mutex<Vec<Handler>>>,
}
