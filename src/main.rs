use crate::cli::command::{Command, MainCommand};

pub mod cli;
pub mod utils;

fn main() {
    let mut main: MainCommand = MainCommand::default();
    main.add_option("-a", "app", "create a tank", false)
        .add_option("-ab", "ape", "create a tank", false)
        .add_option("-a", "apde", "uy a tank", false);

    dbg!(main);
}
