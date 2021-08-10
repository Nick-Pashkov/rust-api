mod server;
use server::Server;
use server::BodyTypes;
use std::thread;
use std::time::Duration;

use serde_json::json;

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 4000);

    server.get("/", |request, response| {
        let res = json!({
            "name": "Nikita Pashkov"
        });
        response.send(BodyTypes::Json(res));
    });

    server.get("/sleep", |request, response| {
        thread::sleep(Duration::from_secs(5));
        response.send(BodyTypes::Text("Ok".to_string()));
    });

    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
