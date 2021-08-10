use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

use crate::server::BodyTypes;

#[allow(dead_code)]
pub struct Response<'a> {
    pub headers: HashMap<String, String>,
    pub status: u16,
    body: BodyTypes,
    stream: &'a TcpStream,
}

impl <'a> Response <'_> {
    pub fn new(stream: &TcpStream) -> Response {
        Response {
            headers: HashMap::new(),
            status: 200,
            body: BodyTypes::Text("".to_string()),
            stream,
        }
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }

    pub fn send(&mut self, data: BodyTypes) -> String {
        let version = "HTTP/1.1";
        let mut headers = String::from("");

        let body: String;
        match data {
            BodyTypes::Text(b) => {
                self.set_header("Content-Type", "text/plain");
                body = b;
            },
            BodyTypes::Json(b) => {
                self.set_header("Content-Type", "application/json");
                body = b.to_string();
            },
        }

        for (key, val) in self.headers.iter() {
            let new_header = format!("{}: {}\r\n", key.to_string(), val.to_string());
            headers.push_str(&new_header);
        }

        let response = &format!("{} {} \r\n{}\r\n{}", version, self.status, headers, body);
        self.stream.write(response.as_bytes()).unwrap();
        
        String::from(response)
    }
}