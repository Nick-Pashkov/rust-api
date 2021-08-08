mod server;

use server::Server;

fn main() {
    // Create server
    let server = Server::new("127.0.0.1", 4000);

    server.get("/", || {
        println!("Get / method");
    });

    // Start listening
    server.listen(|addr| {
        println!("Server listening on {}", addr);
    });
}
