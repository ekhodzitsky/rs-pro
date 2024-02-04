pub mod dynamic_impl;
pub mod static_impl;

pub trait Coffee {
    fn cost(&self) -> u32;
}
