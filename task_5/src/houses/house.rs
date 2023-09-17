use crate::{devices::device::Device, rooms::room::Room};

pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(name: String) -> House {
        House {
            name,
            rooms: Vec::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn create_report(&self, devices: Vec<Box<dyn Device>>) -> Result<String, String> {
        if devices.is_empty() {
            return Err(String::from(
                "Cannot create a report without specifying devices.",
            ));
        }

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
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDevice {
        name: String,
    }

    impl Device for MockDevice {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_info(&self) -> String {
            String::from("Mock Device")
        }
    }

    #[test]
    fn test_new_house() {
        let house = House::new(String::from("Test House"));
        assert_eq!(house.name, "Test House");
        assert!(house.rooms.is_empty());
    }

    #[test]
    fn test_add_room() {
        let mut house = House::new(String::from("Test House"));
        let room = Room::new(String::from("Test Room"));
        house.add_room(room);

        assert_eq!(house.rooms.len(), 1);
    }

    #[test]
    fn test_create_report() {
        let mut house = House::new(String::from("Test House"));
        let mut room1 = Room::new(String::from("Living Room"));
        let mut room2 = Room::new(String::from("Bedroom"));

        let mock_device1 = MockDevice {
            name: String::from("Mock Device 1"),
        };
        let mock_device2 = MockDevice {
            name: String::from("Mock Device 2"),
        };

        room1.add_device(Box::new(mock_device1));
        room2.add_device(Box::new(mock_device2));
        house.add_room(room1);
        house.add_room(room2);

        let devices: Vec<Box<dyn Device>> = vec![
            Box::new(MockDevice {
                name: String::from("Mock Device 1"),
            }),
            Box::new(MockDevice {
                name: String::from("Mock Device 2"),
            }),
            Box::new(MockDevice {
                name: String::from("Non-existent Device"),
            }),
        ];

        let report = house.create_report(devices).unwrap();

        assert!(report.contains("Room \"Living Room\". Mock Device"));
        assert!(report.contains("Room \"Bedroom\". Mock Device"));
        assert!(report.contains("Device \"Non-existent Device\" not found in the house"));
    }
}
