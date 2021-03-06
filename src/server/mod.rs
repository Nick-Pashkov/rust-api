use std::sync::{Arc, RwLock};
use serde_json::{Value as JsonValue};

mod server;
pub mod request;
pub mod response;
mod threading;
pub mod errors;
mod parser;

use request::{Request, RequestMethods};
use response::Response;

type HandlerFunction = Box<dyn Fn(&Request, &mut Response) -> Result<(), errors::RequestError> + Send + Sync + 'static>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BodyTypes {
    Text(&'static str),
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
