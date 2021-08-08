mod server;

use server::Server;

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 4000);

    server.get("/", |request, response| {
        println!("Get {}", request.path);
        response.send("Hola".to_string());
    });

    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
