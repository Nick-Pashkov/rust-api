mod server;
use server::{Server, BodyTypes};
use std::thread;
use std::time::Duration;

use serde_json::json;
use serde_json::{ Value as JsonValue };

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 4000);

    // Test middleware
    server.middleware(|request, response| {
        println!("This middleware runs on any request");
    });

    server.get("/", |request, response| {
        let res = json!("Hola");
        println!("{}", res);
        response.send_v2(res);
    });

    server.post("/create", |request, response| {
        response.send(BodyTypes::Text("Test".to_string()));
    });

    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
