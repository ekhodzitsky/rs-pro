mod domain;
mod server;

use domain::socket::Socket;
use server::server_builder::ServerBuilder;

fn main() {
    ServerBuilder::new("127.0.0.1".to_string(), 8080)
        .with_device(Box::new(Socket::new("Smart Socket".to_string())))
        .build()
        .start()
}
