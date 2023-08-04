use smart_house::devices::device::Device;
use smart_house::devices::socket::Socket;
use smart_house::devices::thermometer::Thermometer;
use smart_house::houses::house::House;
use smart_house::rooms::room::Room;

fn main() {
    let socket1 = Box::new(Socket::new("Living Room Socket".to_string()));
    let socket2 = Box::new(Socket::new("Bedroom socket".to_string()));
    let thermometer1 = Box::new(Thermometer::new("Living Room Thermometer".to_string()));

    let mut living_room = Room::new("Living Room".to_string());
    let mut bedroom = Room::new("Bedroom".to_string());
    let mut my_house = House::new("My House".to_string());

    living_room.add_device(socket1);
    living_room.add_device(thermometer1);
    bedroom.add_device(socket2);
    my_house.add_room(living_room);
    my_house.add_room(bedroom);

    let report_devices: Vec<Box<dyn Device>> = vec![
        Box::new(Socket::new("Living Room Socket".to_string())),
        Box::new(Thermometer::new("Living Room Thermometer".to_string())),
        Box::new(Thermometer::new("Unknown Thermometer".to_string())),
    ];

    let report = my_house.create_report(report_devices);

    println!("{}", report);
}
