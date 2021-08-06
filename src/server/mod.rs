use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod threading;
use threading::ThreadPool;

pub struct Server {
    host: String,
    port: u16,
    pool_size: usize,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Server {
        let host = String::from(host);

        let pool_size = 4;

        Server { host, port, pool_size }
    }

    pub fn set_pool_size(&mut self, size: usize) {
        self.pool_size = size;
    }

    pub fn listen(&self) {
        let addr = format!("{}{}{}", self.host, ":", self.port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(self.pool_size);

        for stream in listener.incoming() {
            pool.execute(|| {
                handler(stream.unwrap());
            });
        }
    }
}

fn handler(mut stream: TcpStream) {

    let (request, size) = read_stream(&mut stream);
    println!("Bytes: {:?} Size: {}", String::from_utf8_lossy(&request), size);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
