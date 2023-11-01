use crate::domain::device::Device;

use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub struct Server {
    device: Box<dyn Device>,
    ip: String,
    port: u16,
}

impl Server {
    pub fn start(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    let bytes_read = stream.read(&mut buffer).unwrap();

                    let command = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Incoming command: {}", command);

                    let response = match command.trim() {
                        "ON" => {
                            self.device.toggle();
                            self.device.get_info()
                        }
                        "OFF" => {
                            self.device.toggle();
                            self.device.get_info()
                        }
                        "INFO" => self.device.get_info(),
                        _ => format!("Invalid command: {}\n", command).to_string(),
                    };

                    println!("Sent response: {}\n", response);
                    stream.write(response.as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

pub struct ServerBuilder {
    ip: String,
    port: u16,
    device: Option<Box<dyn Device>>,
}

impl ServerBuilder {
    pub fn new(ip: String, port: u16) -> Self {
        ServerBuilder {
            ip,
            port,
            device: None,
        }
    }

    pub fn with_device(mut self, device: Box<dyn Device>) -> Self {
        self.device = Some(device);
        self
    }

    pub fn build(self) -> Server {
        Server {
            device: self.device.unwrap(),
            ip: self.ip,
            port: self.port,
        }
    }
}
