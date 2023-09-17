use crate::devices::device::Device;

pub struct Room {
    name: String,
    devices: Vec<Box<dyn Device>>,
}

impl Room {
    pub fn new(name: String) -> Room {
        Room {
            name,
            devices: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    pub fn get_device_by_name(&self, name: &str) -> Option<&dyn Device> {
        self.devices
            .iter()
            .find(|device| device.get_name() == name)
            .map(|device| device.as_ref())
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
    fn test_new_room() {
        let room = Room::new(String::from("Test Room"));
        assert_eq!(room.name, "Test Room");
        assert!(room.devices.is_empty());
    }

    #[test]
    fn test_get_name() {
        let room = Room::new(String::from("Test Room"));
        assert_eq!(room.get_name(), "Test Room");
    }

    #[test]
    fn test_add_device() {
        let mut room = Room::new(String::from("Test Room"));
        let mock_device = MockDevice {
            name: String::from("Mock Device"),
        };
        room.add_device(Box::new(mock_device));

        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_get_device_by_name() {
        let mut room = Room::new(String::from("Test Room"));
        let mock_device = MockDevice {
            name: String::from("Mock Device"),
        };
        room.add_device(Box::new(mock_device));

        let device = room.get_device_by_name("Mock Device");
        assert!(device.is_some());

        let device_name = device.unwrap().get_name();
        assert_eq!(device_name, "Mock Device");
    }
}
