pub trait Device {
    fn get_name(&self) -> &str;
    fn get_info(&self) -> String;
}
