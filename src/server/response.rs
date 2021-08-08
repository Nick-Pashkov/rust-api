use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

pub struct Response<'a> {
    headers: HashMap<String, String>,
    pub status: u16,
    body: String,
    stream: &'a TcpStream,
}

impl <'a> Response <'_> {
    pub fn new(stream: &TcpStream) -> Response {
        Response {
            headers: HashMap::new(),
            status: 200,
            body: "".to_string(),
            stream,
        }
    }

    pub fn send(&mut self, data: String) {
        let version = "HTTP/1.1";
        let response = &format!("{} {} \r\n\r\n{}", version, self.status, data);
        self.stream.write(response.as_bytes()).unwrap();
    }
}