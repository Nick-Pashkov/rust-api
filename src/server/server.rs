use std::io::prelude::*;
use std::io::{BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use regex::Regex;

use crate::server::{Server, Handler, HandlerFunction};
use crate::server::threading::ThreadPool;
use crate::server::request::{Request, RequestMethods};
use crate::server::response::Response;
use crate::server::errors::RequestError;
use crate::server::parser;

#[allow(dead_code)]
impl Server {
    pub fn new(host: &str, port: u16) -> Server {
        let host = String::from(host);

        let pool_size = 4;

        let handlers = Arc::new(RwLock::new(Vec::new()));

        let middleware = Arc::new(RwLock::new(Vec::new()));

        Server { host, port, pool_size, handlers, middleware }
    }

    pub fn listen<F>(&self, f: F) where F: FnOnce(&String) {
        let addr = &format!("{}{}{}", self.host, ":", self.port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(self.pool_size);

        f(&addr);

        for stream in listener.incoming() {
            let handlers_clone = Arc::clone(&self.handlers);
            let middleware_clone = Arc::clone(&self.middleware);
            pool.execute(move || {
                handler(stream.unwrap(), handlers_clone, middleware_clone);
            });
        }
    }

    pub fn middleware<F>(&mut self, f: F) where F: Fn(&Request, &mut Response) -> Result<(), RequestError> + Send + Sync + 'static {
        let mut middleware = self.middleware.write().unwrap();
        middleware.push(Box::new(f))
    }

    pub fn get<F>(&mut self, path: &str, f: F) where F: Fn(&Request, &mut Response) -> Result<(), RequestError> + Send + Sync + 'static {
        self.create_handler(path, RequestMethods::GET, f);
    }

    pub fn post<F>(&mut self, path: &str, f: F) where F: Fn(&Request, &mut Response) -> Result<(), RequestError> + Send + Sync + 'static {
        self.create_handler(path, RequestMethods::POST, f);
    }

    fn create_handler<F>(&mut self, path: &str, method: RequestMethods, f: F) where F: Fn(&Request, &mut Response) -> Result<(), RequestError> + Send + Sync + 'static {
        let handler = Handler {
            path: path.to_string(),
            method,
            handler: Box::new(f),
        };

        let mut handlers = self.handlers.write().unwrap();
        handlers.push(handler)
    }
}

fn handler(mut stream: TcpStream, handlers: Arc<RwLock<Vec<Handler>>>, middlewares: Arc<RwLock<Vec<HandlerFunction>>>) {

    let mut reader = BufReader::new(&stream);
    
    let request = Request::new(&mut reader);
    //std::process::exit(0);

    match request {
        Ok(mut request) => {
            let mut handler_exists = false;
            
            let mut response = Response::new(&stream);

            let middlewares = middlewares.read().unwrap();
            let handlers = handlers.read().unwrap();

            // Run middlewares
            for middleware in middlewares.iter() {
                match middleware(&request, &mut response) {
                    Ok(res) => res,
                    Err(err) => { println!("{}", err); }
                }
            }

            for handler in handlers.iter() {
                let handler_regex = Regex::new(&parser::handler_pattern(&handler.path)).unwrap();

                let caps = match handler_regex.captures(&request.path) {
                    None => continue,
                    Some(caps) => caps
                };

                if request.method == handler.method {
                    handler_exists = true;
                    let handler = &handler.handler;

                    for var in handler_regex.capture_names().flatten().filter_map(|n| Some((n, caps.name(n)?.as_str()))) {
                        request.params.insert(var.0.to_string(), var.1.to_string());
                    }

                    match handler(&request, &mut response) {
                        Ok(_) => { break },
                        Err(err) => println!("{}", err)
                    }
                    break;
                }
            }

            if !handler_exists {
                response.status = 404;
                let result = format!("Cannot {} {}", request.method, request.path);
                response.write(&result.as_bytes().to_vec());
            }

            stream.flush().unwrap();
        }
        Err(e) => { println!("{}", e); }
    }
}
