use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::io::{self, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use crate::server::{Server, Handler, BodyTypes, HandlerFunction};
use crate::server::threading::ThreadPool;
use crate::server::request::{Request, RequestMethods};
use crate::server::response::Response;

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
            let cloneHandlers = Arc::clone(&self.handlers);
            let cloneMiddleware = Arc::clone(&self.middleware);
            pool.execute(move || {
                handler(stream.unwrap(), cloneHandlers, cloneMiddleware);
            });
        }
    }

    pub fn middleware<F>(&mut self, f: F) where F: Fn(&Request, &mut Response) + Send + Sync + 'static {
        let mut middleware = self.middleware.write().unwrap();
        middleware.push(Box::new(f))
    }

    pub fn get<F>(&mut self, path: &str, f: F) where F: Fn(&Request, &mut Response) + Send + Sync + 'static {
        self.create_handler(path, RequestMethods::GET, f);
    }

    pub fn post<F>(&mut self, path: &str, f: F) where F: Fn(&Request, &mut Response) + Send + Sync + 'static {
        self.create_handler(path, RequestMethods::POST, f);
    }

    fn create_handler<F>(&mut self, path: &str, method: RequestMethods, f: F) where F: Fn(&Request, &mut Response) + Send + Sync + 'static {
        let handler = Handler {
            path: path.to_string(),
            method,
            handler: Box::new(f),
        };

        let mut handlers = self.handlers.write().unwrap();
        handlers.push(handler)
    }
}

fn get_length(reader: &mut BufReader<TcpStream>) -> usize {
    let mut str_buff = String::new();
    loop {
        reader.read_line(&mut str_buff).unwrap();
        if str_buff.starts_with("Content-Length") {
            let res: Vec<&str> = str_buff.split(":").collect();

            reader.read_line(&mut String::new()).unwrap();

            return res[1].trim().parse::<usize>().unwrap();
        }
        str_buff = "".to_string();
    }
}

fn handler(mut stream: TcpStream, handlers: Arc<RwLock<Vec<Handler>>>, middlewares: Arc<RwLock<Vec<HandlerFunction>>>) {

    let mut reader = BufReader::new(&stream);
    
    let request = Request::new(&mut reader);
    //std::process::exit(0);

    match request {
        Ok(request) => {
            let mut handler_exists = false;
            
            let mut response = Response::new(&stream);

            let middlewares = middlewares.read().unwrap();
            let handlers = handlers.read().unwrap();

            // Run middlewares
            for middleware in middlewares.iter() {
                middleware(&request, &mut response);
            }

            for handler in handlers.iter() {
                if request.path == handler.path && request.method == handler.method {
                    handler_exists = true;
                    let handler = &handler.handler;

                    handler(&request, &mut response);
                    break;
                }
            }

            if !handler_exists {
                response.status = 404;
                response.send(BodyTypes::Text(format!("Cannot {} {}", request.method, request.path)));
            }

            stream.flush().unwrap();
        }
        Err(e) => { println!("{}", e); }
    }
}

fn read_stream(stream: &mut TcpStream) -> (Vec<u8>, usize) {

    let buffer_size = 1024;
    let mut request_buffer = vec![];
    let mut request_length = 0usize;

    loop {
        let mut buffer = vec![0; buffer_size];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    request_length += n;

                    if n < buffer_size {
                        println!("Before {}", request_buffer.len());
                        request_buffer.append(&mut buffer[..n].to_vec());
                        println!("After {}", request_buffer.len());
                        break;
                    } else {
                        println!("Before {}", request_buffer.len());
                        request_buffer.append(&mut buffer);
                        println!("After {}", request_buffer.len());
                    }
                }
            }
            Err(e) => {
                println!("Error in reading stream data: {:?}", e);
                break;
            }
        }
    }

    (request_buffer, request_length)
}