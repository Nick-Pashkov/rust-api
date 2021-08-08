use std::str::FromStr;
use std::result::Result;
use std::collections::HashMap;

use crate::server::errors::RequestError;

pub struct Request {
    method: RequestMethods,
    pub path: String,
    headers: HashMap<String, String>,
    body: String,
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

impl Request {
    pub fn new(input: String) -> Result<Request, RequestError> {

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
        
        let body = input.collect::<Vec<&str>>().join("\n");

        Ok(Request { method, path, headers, body })
    }
}