use crate::cli::command::{Command, MainCommand};

pub mod cli;
pub mod utils;

fn main() {
    let mut main: MainCommand = MainCommand::default();
    main.add_option("-a", "--a", "create a tank");
    dbg!(main);
}
