use std::net::UdpSocket;
use std::str;
use std::thread;

pub fn run() {
    thread::spawn(|| {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("Не удалось привязать сокет");
        println!("Сокет запущен, ожидает данные на 127.0.0.1:34254...");

        loop {
            let mut buffer = [0u8; 1024];
            match socket.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    let received_data =
                        str::from_utf8(&buffer[..size]).expect("Ошибка при декодировании данных");
                    println!("Получены данные о температуре: {}", received_data);
                }
                Err(e) => eprintln!("Ошибка при получении данных: {}", e),
            }
        }
    });
}
