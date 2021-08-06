mod server;
use server::Server;

fn main() {
    // Create server
    let mut server = Server::new("127.0.0.1", 7878);
    server.set_pool_size(8);

    // Start listening
    server.listen();

    println!("Shutting down");
}
