pub trait Device {
    fn toggle(&mut self) -> bool;
    fn get_info(&self) -> String;
}