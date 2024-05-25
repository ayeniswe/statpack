use crate::cli::{
    command::{Command, MainCommand},
    option::{
        CommandOptionKwargsBuilder,
        CommandOptionType::{Int, Text},
    },
};

pub mod cli;
pub mod utils;

fn main() {
    let mut main: MainCommand = MainCommand::default();
    let kwargs = CommandOptionKwargsBuilder::new()
        .set_default(Int(34))
        .set_choices(vec![Text("re".to_string())])
        .build();
    main.create_option_kwargs("a", "create a tank", kwargs);
    dbg!(main);
}
