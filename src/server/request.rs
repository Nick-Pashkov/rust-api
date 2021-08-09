use std::str::FromStr;
use std::result::Result;
use std::collections::HashMap;
use std::fmt;
use serde_json::{Value as JsonValue};

use crate::server::errors::RequestError;
use crate::server::BodyTypes;

pub struct Request {
    pub method: RequestMethods,
    pub path: String,
    headers: HashMap<String, String>,
    pub body: BodyTypes,
    pub size: usize,
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

impl Request {
    pub fn new(input: String, size: usize) -> Result<Request, RequestError> {

        let mut input = input.lines();
        
        // Request line
        let line: Vec<&str> = input.next().expect("Invalid HTTP Request").split_whitespace().collect();
        
        let method = line[0].parse::<RequestMethods>()?;

        let path = String::from(line[1]);
        let mut headers = HashMap::new();

        // Get all headers in a loop
        loop {
            match input.next() {
                Some(header) => {
                    if header == "" {
                        break;
                    }

                    let header: Vec<&str> = header.split(": ").collect(); 
                    headers.insert(String::from(header[0]), String::from(header[1]));
                }
                None => break
            }
        }
        
        let body_str = input.collect::<Vec<&str>>().join("\n");
        let content_type = headers.get(&"Content-Type".to_string());
        let body: BodyTypes;

        match content_type {
            Some(content_type) => {
                body = parse_body(body_str, String::from(content_type));
            }
            None => {
                body = BodyTypes::Text(body_str);
            }
        }

        Ok(Request { method, path, headers, body, size })
    }
}

fn parse_body(body: String, content_type: String) -> BodyTypes {
    
    let content_type = String::from(content_type);
    if content_type == "application/json" {
        return BodyTypes::Json(serde_json::from_str(&body).unwrap());
    } else {
        return BodyTypes::Text(String::from(body));
    }
}