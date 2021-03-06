mod server;
use server::{Server, BodyTypes};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

use serde_json::json;
mod logger;

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 4000);
    server.middleware(logger::log_to_file);
/*
    server.post("/upload", |request, response| {
        let body = request.body_as_bytes();

        let content_type = request.get_header("Content-Type")?;

        let auth = match request.get_header("Authorization") {
            Ok(auth) => auth,
            Err(err) => { return Err(err.into()); }
        };

        let mut file = File::create("public/test.jpeg")?;
        file.write_all(body)?;

        println!("{}", content_type);
        //println!("{:?}", body);
        response.set_header("Content-Type", content_type);

        Ok(response.write(body))
    });
*/
    server.get("/users", |request, response| {
        let body = request.body_as_bytes();

        let res = json!({
            "id": "123123",
            "name": "Nick Pashkov"
        });
        Ok(response.json(res))
    });

    server.get("/users/{id}", |request, response| {
        let body = request.body_as_bytes();
        Ok(response.write(&"Single User".as_bytes().to_vec()))
    });

    server.get("/search", |request, response| {
        Ok(response.write(&"Search handler".as_bytes().to_vec()))
    });
    /*
    server.get("/.*", |request, response| {
        let body = request.body_as_bytes();
        Ok(response.send(BodyTypes::Text("Ok".to_string())))
    });
    */
    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
