use super::device::Device;

pub struct Thermometer {
    name: String,
    is_on: bool,
    temperature: f32,
}

impl Thermometer {
    pub fn new(name: String) -> Thermometer {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_thermometer() {
        let thermometer = Thermometer::new(String::from("Test Thermometer"));
        assert_eq!(thermometer.name, "Test Thermometer");
        assert_eq!(thermometer.is_on, false);
        assert_eq!(thermometer.temperature, 0.0);
    }

    #[test]
    fn test_get_name() {
        let thermometer = Thermometer::new(String::from("Test Thermometer"));
        assert_eq!(thermometer.get_name(), "Test Thermometer");
    }

    #[test]
    fn test_get_info() {
        let thermometer = Thermometer::new(String::from("Test Thermometer"));
        assert_eq!(
            thermometer.get_info(),
            "Device \"Test Thermometer\" is not working. Temperature: 0"
        );
    }
}
