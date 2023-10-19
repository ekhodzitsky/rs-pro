mod builder;
mod command;
mod newtype;
mod raii;
mod strategy;

fn main() {
    builder::demo();
    command::demo();
    newtype::demo();
    raii::demo();
    strategy::demo();
}
