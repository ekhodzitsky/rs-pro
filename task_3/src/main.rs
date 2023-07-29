trait Device {
    fn get_name(&self) -> &str;
    fn get_info(&self) -> String;
}

struct Socket {
    name: String,
    is_on: bool,
    power_consumption: f32,
}

impl Socket {
    fn new(name: String) -> Socket {
        Socket {
            name,
            is_on: false,
            power_consumption: 0.0,
        }
    }
}

impl Device for Socket {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_info(&self) -> String {
        format!(
            "Device \"{}\" is {}. Power consumption: {}",
            &self.get_name(),
            match &self.is_on {
                true => String::from("working"),
                false => String::from("not working"),
            },
            &self.power_consumption
        )
    }
}

struct Thermometer {
    name: String,
    is_on: bool,
    temperature: f32,
}

impl Thermometer {
    fn new(name: String) -> Thermometer {
        Thermometer {
            name,
            is_on: false,
            temperature: 0.0,
        }
    }
}

impl Device for Thermometer {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_info(&self) -> String {
        format!(
            "Device \"{}\" is {}. Temperature: {}",
            &self.get_name(),
            match &self.is_on {
                true => String::from("working"),
                false => String::from("not working"),
            },
            &self.temperature
        )
    }
}

struct Room {
    name: String,
    devices: Vec<Box<dyn Device>>,
}

impl Room {
    fn new(name: String) -> Room {
        Room {
            name,
            devices: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    fn get_device_by_name(&self, name: &str) -> Option<&dyn Device> {
        self.devices
            .iter()
            .find(|device| device.get_name() == name)
            .map(|device| device.as_ref())
    }
}

struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    fn new(name: String) -> House {
        House {
            name,
            rooms: Vec::new(),
        }
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn create_report(&self, devices: Vec<Box<dyn Device>>) -> String {
        let mut report = format!("Report for house \"{}\":\n", self.name);

        for device in devices {
            let mut count = 0;
            let device_name = device.get_name();

            for room in &self.rooms {
                if let Some(found) = room.get_device_by_name(device_name) {
                    report.push_str(
                        format!("Room \"{}\". {}\n", room.get_name(), found.get_info()).as_str(),
                    );
                    count += 1;
                }
            }

            if count == 0 {
                report.push_str(
                    format!(
                        "Device \"{}\" not found in the house \"{}\".\n",
                        device_name, self.name
                    )
                    .as_str(),
                )
            }
        }
        report
    }
}

fn main() {
    let mut socket1 = Box::new(Socket::new("Living Room Socket".to_string()));

    socket1.is_on = true;
    socket1.power_consumption = 1.3;

    let socket2 = Box::new(Socket::new("Bedroom socket".to_string()));

    let mut thermometer1 = Box::new(Thermometer::new("Living Room Thermometer".to_string()));
    thermometer1.is_on = true;
    thermometer1.temperature = 36.6;

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
