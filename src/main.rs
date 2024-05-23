use crate::cli::command::{Command, MainCommand};

pub mod cli;
pub mod utils;

fn main() {
    let mut main: MainCommand = MainCommand::default();
    main.create_option("app", "create a tank", false)
        .create_option("ape", "create a tank", false)
        .create_option("apde", "uy a tank", false);

    dbg!(main);
}
