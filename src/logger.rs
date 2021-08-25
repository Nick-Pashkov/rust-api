use crate::server::request::{Request, RequestMethods};
use crate::server::response::Response;
use crate::server::errors::RequestError;

use chrono;

use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn log_to_file(request: &Request, response: &mut Response) -> Result<(), RequestError> {
    let mut file = OpenOptions::new().append(true).create(true).open("public/logs.log")?;

    let mut query_str = String::from("?");
    for (key, value) in request.query.iter() {
        query_str += &format!("{}={}&", key, value)
    }
    query_str.remove(query_str.len() - 1);

    let line = format!("{:?}: {} {}{}\n", chrono::offset::Local::now().naive_local(), request.method, request.path, query_str);
    file.write_all(line.as_bytes())?;

    Ok(println!("{}", line))
}