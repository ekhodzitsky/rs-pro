use rand::Rng;
use std::net::UdpSocket;
use std::{thread, time::Duration};

pub struct Thermometer {
    name: String,
    socket: UdpSocket,
    server_address: String,
}

impl Thermometer {
    pub fn new(name: &str, server_address: &str) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Не удалось создать сокет для термометра");
        Thermometer {
            name: name.to_string(),
            socket,
            server_address: server_address.to_string(),
        }
    }

    pub fn send(&self) {
        let mut rng = rand::thread_rng();
        loop {
            let temperature = rng.gen_range(10.0..30.0);
            let message = format!("{}: {:.1}°C", self.name, temperature);
            self.socket
                .send_to(message.as_bytes(), &self.server_address)
                .expect("Не удалось отправить данные о температуре");
            thread::sleep(Duration::from_secs(rng.gen_range(1..10)));
        }
    }
}
