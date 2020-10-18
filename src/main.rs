pub mod utils;
pub mod commands;
pub mod config;
pub mod constants;

fn main() {
    utils::system::run_or_interrupt(commands::main);
}
