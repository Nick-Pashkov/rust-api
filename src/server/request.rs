use std::str::FromStr;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::result::Result;
use std::collections::HashMap;
use std::fmt;
use serde_json::{Value as JsonValue};

use crate::server::errors::RequestError;

#[allow(dead_code)]
pub struct Request {
    pub method: RequestMethods,
    pub path: String,
    headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    body: Vec<u8>,
   // pub size: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RequestMethods {
    GET, POST, PUT, DELETE, PATCH
}

impl FromStr for RequestMethods {
    type Err = RequestError;

    fn from_str(s: &str) -> Result<RequestMethods, RequestError> {
        match s {
            "GET" => Ok(RequestMethods::GET),
            "POST" => Ok(RequestMethods::POST),
            "PUT" => Ok(RequestMethods::PUT),
            "DELETE" => Ok(RequestMethods::DELETE),
            "PATCH" => Ok(RequestMethods::PATCH),
            _ => Err(RequestError::new("Invalid request method", s))
        }
    }
}

impl fmt::Display for RequestMethods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
impl Request {
    pub fn new(reader: &mut BufReader<&TcpStream>) -> Result<Request, RequestError> {

        let (method, path) = get_line(reader);

        let headers = get_headers(reader);

        let content_length = match headers.get("Content-Length") {
            Some(value) => value.parse().unwrap(),
            None => 0
        };

        let (path, query) = parse_query(path);

        println!("{}", path);

        let body = get_body(reader, content_length);
        let params = HashMap::new();

        Ok(Request { method, path, headers, body, params, query })
    }

    pub fn body_as_bytes(&self) -> &Vec<u8> {
        return &self.body;
    }

    pub fn body_as_json(&self) -> JsonValue {
        let body_str = String::from_utf8_lossy(&self.body);

        serde_json::from_str(&body_str).unwrap()
    }

    pub fn get_header(&self, name: &str) -> Result<&String, RequestError> {
        self.headers.get(name).ok_or(RequestError::new("Header does not exist", name))
    }
}

fn get_line(reader: &mut BufReader<&TcpStream>) -> (RequestMethods, String) {
    let mut line_str = String::new();
    reader.read_line(&mut line_str).unwrap();

    let parts: Vec<&str> = line_str.split(" ").collect();
    
    (RequestMethods::from_str(parts[0]).unwrap(), parts[1].to_string())
}

fn get_headers(reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut str_buff = String::new();
    loop {
        reader.read_line(&mut str_buff).unwrap();

        if str_buff == "\r\n" {
            return result;
        }

        let header: Vec<&str> = str_buff.split(": ").collect();
        result.insert(String::from(header[0]), String::from(header[1].trim()));
        str_buff = String::from("");
    }
}

fn get_body(reader: &mut BufReader<&TcpStream>, length: usize) -> Vec<u8> {
    let mut buffer = vec![0; length];
    reader.read_exact(&mut buffer).unwrap();
    return buffer;
}

fn parse_query(input: String) -> (String, HashMap<String, String>) {
    let mut params: HashMap<String, String> = HashMap::new();
    if input == "" {
        return ("".to_string(), params);
    }
    let path_and_params: Vec<&str> = input.split("?").collect();
    if path_and_params.len() == 1 {
        return (path_and_params[0].to_string(), params);
    }

    let params_vec: Vec<&str> = path_and_params[1].split("&").collect();
    for param in params_vec.iter() {
        let param: Vec<&str> = param.split("=").collect();
        let key = param[0];
        let value = if param.len() > 1 { param[1] } else { "" };

        params.insert(key.to_string(), value.to_string());
    }

    (path_and_params[0].to_string(), params)
}
