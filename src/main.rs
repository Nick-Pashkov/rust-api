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

    server.get("/", |request, response| {
        let res = json!("Hola");
        println!("{}", res);
        response.send(BodyTypes::Json(res));
    });

    server.post("/create", |request, response| {
        //println!("{:?}", request.body);
        response.send(BodyTypes::Text("Test".to_string()));
    });

    server.post("/upload", |request, response| {
        let body = request.body_as_bytes();

        let content_type = request.get_header("Content-Type").unwrap();

        let mut file = File::create("public/test.jpg").unwrap();
        file.write_all(body).unwrap();

        println!("{}", content_type);
        //println!("{:?}", body);
        response.set_header("Content-Type", content_type);
        response.write(body);
    });

    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
