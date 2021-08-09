use std::sync::{Arc, Mutex};
use serde_json::{Value as JsonValue};

mod server;
mod request;
mod response;
mod threading;
mod errors;

use request::RequestMethods;
use request::Request;
use response::Response;

#[derive(Debug)]
pub enum BodyTypes {
    Text(String),
    Json(JsonValue),
}

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
