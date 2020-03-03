use http_server::server::server::Server;

fn main() {
    let server = Server::new();
    println!("starting server...");
    server.run()
}
