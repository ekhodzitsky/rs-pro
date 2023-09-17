use super::device::Device;

pub struct Socket {
    name: String,
    is_on: bool,
    power_consumption: f32,
}

impl Socket {
    pub fn new(name: String) -> Socket {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_socket() {
        let socket = Socket::new(String::from("Test Socket"));
        assert_eq!(socket.name, "Test Socket");
        assert_eq!(socket.is_on, false);
        assert_eq!(socket.power_consumption, 0.0);
    }

    #[test]
    fn test_get_name() {
        let socket = Socket::new(String::from("Test Socket"));
        assert_eq!(socket.get_name(), "Test Socket");
    }

    #[test]
    fn test_get_info() {
        let socket = Socket::new(String::from("Test Socket"));
        assert_eq!(
            socket.get_info(),
            "Device \"Test Socket\" is not working. Power consumption: 0"
        );
    }
}
