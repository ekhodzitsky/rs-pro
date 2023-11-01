use crate::domain::device::Device;

#[derive(Debug)]
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
    fn toggle(&mut self) -> bool {
        self.is_on = !self.is_on;
        self.power_consumption = if self.is_on { 10.0 } else { 0.0 };
        self.is_on
    }

    fn get_info(&self) -> String {
        format!(
            "Name: {}; Status: {}; Power Consumption: {}\n",
            self.name, self.is_on, self.power_consumption
        )
    }
}
