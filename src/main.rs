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
    let mut builder = CommandOptionKwargsBuilder::new();
    let binding = Text("re".to_string());
    let choices = &mut vec![&binding];
    let kwargs = builder.set_default(&Int(34)).set_choices(choices).build();
    main.create_option_kwargs("a", "create a tank", &kwargs);
    dbg!(main);
}
