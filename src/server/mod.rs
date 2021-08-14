use std::sync::{Arc, RwLock};
use serde_json::{Value as JsonValue};

mod server;
mod request;
mod response;
mod threading;
mod errors;

use request::RequestMethods;
use request::Request;
use response::Response;

type HandlerFunction = Box<dyn Fn(&Request, &mut Response) + Send + Sync + 'static>;

#[derive(Debug)]
pub enum BodyTypes {
    Text(String),
    Json(JsonValue),
    Bytes(Vec<u8>)
}

pub struct Handler {
    method: RequestMethods,
    path: String,
    handler: HandlerFunction,
}

pub struct Server {
    pub host: String,
    pub port: u16,
    pool_size: usize,
    handlers: Arc<RwLock<Vec<Handler>>>,
    middleware: Arc<RwLock<Vec<HandlerFunction>>>
}
