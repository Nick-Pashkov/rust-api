mod server;
use server::{Server, BodyTypes};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

use serde_json::json;

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 4000);
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
        println!("{:?}", request.params);
        Ok(response.write(&"Multiple Users".as_bytes().to_vec()))
    });

    server.get("/users/{id}", |request, response| {
        let body = request.body_as_bytes();
        println!("{:?}", request.params);
        Ok(response.write(&"Single User".as_bytes().to_vec()))
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
