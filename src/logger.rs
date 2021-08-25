use crate::server::request::{Request, RequestMethods};
use crate::server::response::Response;
use crate::server::errors::RequestError;

use chrono;

use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn log_to_file(request: &Request, response: &mut Response) -> Result<(), RequestError> {
    let mut file = OpenOptions::new().append(true).create(true).open("logs.log")?;

    let line = format!("{:?}: {} {}\n", chrono::offset::Local::now().naive_utc(), request.method, request.path);
    file.write_all(line.as_bytes())?;

    Ok(println!("{}", line))
}