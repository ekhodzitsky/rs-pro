use std::io::{self, prelude::*};
use std::net::TcpStream;

fn main() {
    loop {
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        stream
            .write_all(user_input.as_bytes())
            .expect("Failed to send command");

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .expect("Failed to read response");

        println!("Received response: {}", response);
    }
}
