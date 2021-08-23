use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;
use std::any::Any;

use serde_json::{Value as JsonValue};

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
            body: BodyTypes::Text(""),
            stream,
        }
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }

    pub fn write(&mut self, data: &Vec<u8>) {
        let version = "HTTP/1.1";
        let mut headers = String::from("");

        self.set_header("Content-Length", &data.len().to_string());
        for (key, val) in self.headers.iter() {
            let new_header = format!("{}: {}\r\n", key.to_string(), val.to_string());
            headers.push_str(&new_header);
        }

        let response = &format!("{} {} \r\n{}\r\n", version, self.status, headers);
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.write(data).unwrap();
    }
}
