use std::fmt;
use std::io;
use std::error::Error;

#[derive(Debug)]
pub struct RequestError {
    pub message: String,
    pub orig: String
}

impl RequestError {
    pub fn new(message: &str, param: &str) -> RequestError {
        RequestError { message: message.to_string(), orig: param.to_string() }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {} {}", self.message, self.orig)
    }
}

impl From<io::Error> for RequestError {
    fn from(err: io::Error) -> RequestError {
        RequestError::new(&err.to_string(), "")
    }
}