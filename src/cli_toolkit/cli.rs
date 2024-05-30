use super::{
    command::{Command, _CLICommand},
    option::CommandOption,
};
use std::collections::HashSet;

/// Represents the top-level command-line interface (CLI)
///
/// The `CLI` struct holds the name of the CLI application, along with its commands,
/// options, and a lookup table for ensuring unique option and command identifiers.
#[derive(Default, Debug)]
pub struct CLI<'a> {
    name: &'a str,
    commands: Vec<Command<'a>>,
    options: Vec<CommandOption<'a>>,
    lookup: HashSet<String>,
}
impl<'a> _CLICommand<'a> for CLI<'a> {
    fn commands(&self) -> &Vec<Command> {
        &self.commands
    }
    fn commands_mut(&mut self) -> &mut Vec<Command<'a>> {
        &mut self.commands
    }
    fn options(&self) -> &Vec<CommandOption> {
        &self.options
    }
    fn options_mut(&mut self) -> &mut Vec<CommandOption<'a>> {
        &mut self.options
    }
    fn lookup_mut(&mut self) -> &mut HashSet<String> {
        &mut self.lookup
    }
    fn lookup(&self) -> &HashSet<String> {
        &self.lookup
    }
}
impl<'a> CLI<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
