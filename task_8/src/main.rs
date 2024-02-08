mod socket;
mod thermometer;

use std::thread;
use std::time::Duration;
use thermometer::Thermometer;

fn main() {
    // Запускаем сокет, который будет принимать данные
    socket::run();

    // Даем сокету время на инициализацию
    thread::sleep(Duration::from_secs(1));

    let mut handles = Vec::new();

    for i in 1..=10 {
        let thermometer = Thermometer::new(&format!("Термометр {}", i), "127.0.0.1:34254");
        let handle = thread::spawn(move || {
            thermometer.send();
        });
        handles.push(handle);
    }

    // Ждем завершения работы всех термометров (хотя в данном случае они работают бесконечно)
    for handle in handles {
        handle.join().unwrap();
    }
}
