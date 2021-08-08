use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::server::Server;
use crate::server::Handler;
use crate::server::threading::ThreadPool;
use crate::server::request::Request;
use crate::server::request::RequestMethods;
use crate::server::response::Response;

impl Server {
    pub fn new(host: &str, port: u16) -> Server {
        let host = String::from(host);

        let pool_size = 4;

        let handlers = Arc::new(Mutex::new(Vec::new()));

        Server { host, port, pool_size, handlers }
    }

    pub fn listen<F>(&self, f: F) where F: FnOnce(&String) {
        let addr = &format!("{}{}{}", self.host, ":", self.port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(self.pool_size);

        f(&addr);

        let handlers = Arc::clone(&self.handlers);

        for stream in listener.incoming() {
            let clone = Arc::clone(&handlers);
            pool.execute(move || {
                handler(stream.unwrap(), clone);
            });
        }
    }

    pub fn get<F>(&mut self, path: &str, f: F) where F: Fn(&Request, &mut Response) + Send + Sync + 'static {
        self.create_handler(path, RequestMethods::GET, f);
    }

    fn create_handler<F>(&mut self, path: &str, method: RequestMethods, f: F) where F: Fn(&Request, &mut Response) + Send + Sync + 'static {
        let handler = Handler {
            path: path.to_string(),
            method,
            handler: Box::new(f),
        };

        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(handler)
    }
}

fn handler(mut stream: TcpStream, handlers: Arc<Mutex<Vec<Handler>>>) {

    let (request, size) = read_stream(&mut stream);
    let request = Request::new(String::from_utf8_lossy(&request).to_string());

    match request {
        Ok(request) => {
            println!("Size: {}", size);

            let handlers = handlers.lock().unwrap();
            let mut handler_exists = false;

            let mut response = Response::new(&stream);

            for handler in handlers.iter() {
                if request.path == handler.path && request.method == handler.method {
                    handler_exists = true;
                    let handler = &handler.handler;
                    
                    handler(&request, &mut response);
                }
            }

            if !handler_exists {
                response.status = 404;
                response.send(format!("Cannot {} {}", request.method, request.path));
            }

            stream.flush().unwrap();
        }
        Err(e) => { println!("{}", e); }
    }
}

fn read_stream(stream: &mut TcpStream) -> (Vec<u8>, usize) {
    let buffer_size = 512;
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
                        request_buffer.append(&mut buffer[..n].to_vec());
                        break;
                    } else {
                        request_buffer.append(&mut buffer);
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