mod server;
use server::Server;
use server::BodyTypes;

use serde_json::json;

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 4000);

    server.get("/", |request, response| {
        println!("{:?}", request.body);

        let res = json!({
            "name": "Nikita Pashkov"
        });
        response.send(BodyTypes::Json(res));
    });

    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
