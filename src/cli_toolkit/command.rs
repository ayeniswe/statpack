use super::option::{CommandOption, CommandOptionBuilder, CommandOptionKwargs};
use super::parser::Parser;
use std::borrow::BorrowMut;
use std::collections::HashSet;

/// The `_Command` trait for internal apis `Command` relies on
pub(super) trait _CLICommand<'a> {
    /// Adds an option to the list and updates the lookup table for indexing and preventing collisions.
    /// This ensures that the option is unique and avoids duplicate entries.
    fn add(&mut self, arg: CommandOption<'a>) {
        let short = arg.short.clone();
        let long = arg.long.clone();
        self.lookup_mut().insert(short);
        self.lookup_mut().insert(long);
        self.options_mut().push(arg);
    }
    /// Returns a immutable reference to the commands list
    fn commands(&self) -> &Vec<Command>;
    /// Returns a mutable reference to the commands list
    fn commands_mut(&mut self) -> &mut Vec<Command<'a>>;
    /// Returns a immutable reference to the lookup table
    fn lookup(&self) -> &HashSet<String>;
    /// Returns a mutable reference to the lookup table
    fn lookup_mut(&mut self) -> &mut HashSet<String>;
    /// Returns a mutable reference to the options list
    fn options_mut(&mut self) -> &mut Vec<CommandOption<'a>>;
    /// Returns a immutable reference to the options list
    fn options(&self) -> &Vec<CommandOption>;
}

/// The `Command` trait provides methods for managing command-line commands.
/// It extends the `_Command` trait for internal apis and adds higher-level functionality for
/// creating and managing options with additional configurations.
pub trait CLICommand<'a>: _CLICommand<'a> + Parser {
    /// Creates a new command/subcommand with the specified name.
    ///
    /// ## Returns
    ///
    /// A mutable reference to the newly created command.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut cli = CLI::new("test");
    /// cli.create_command("test-subcommand", "Do the subcommand");
    /// ```
    fn create_command(&mut self, name: &'a str) -> &mut Command<'a> {
        self.commands_mut().push(Command::new(name));
        self.commands_mut().last_mut().unwrap()
    }
    /// Autogen a new command-line option and adds to option list
    ///
    /// This method simplifies the process of building options by automatically generating
    /// the short, and long flags of the option based on the provided option string.
    ///
    /// # Example
    ///
    /// ```
    /// let mut cli = CLI::new();
    /// cli.create_option("verbose", "Enable verbose mode");
    /// ```
    fn create_option(&mut self, option: &str, description: &str) -> &mut Self {
        let arg = CommandOptionBuilder::new()
            .gen_short(option, self.lookup())
            .gen_long(option)
            .set_description(description)
            .build();
        self.add(arg);
        self
    }
    /// Similar to `create_option` but allows the use of predefined extra options
    ///
    /// # Example
    ///
    /// ```
    /// let mut cli = CLI::new();
    /// let kwargs = CommandOptionKwargsBuilder::new()
    /// .set_deprecated()
    /// .set_required();
    /// cli.create_option("verbose", "Enable verbose mode", &kwargs);
    /// ```
    fn create_option_kwargs(
        &mut self,
        option: &str,
        description: &str,
        kwargs: &'a CommandOptionKwargs,
    ) -> &mut Self {
        let arg = CommandOptionBuilder::new()
            .gen_short(option, self.lookup())
            .gen_long(option)
            .set_description(description)
            .set_kwargs(kwargs)
            .build();
        self.add(arg);
        self
    }
    /// This function is useful for defining command-line arguments that the program accepts.
    /// By using this method, you can manually add options with a short and long version,
    /// along with a description to clarify the purpose of the option to the user.
    /// # Example
    ///
    /// ```
    /// let mut cli = CLI::new();
    /// cli.add_option("-v", "--verbose", "Enable verbose mode");
    /// ```
    fn add_option(&mut self, short: &str, long: &str, description: &str) -> &mut Self {
        // Index check if already added
        assert!(
            self.lookup_mut().get(short).is_none() && self.lookup_mut().get(long).is_none(),
            "short and/or long options already exist"
        );
        let arg = CommandOptionBuilder::new()
            .set_short(short)
            .set_long(long)
            .set_description(description)
            .build();
        self.add(arg);
        self
    }
    /// Similar to `add_option` but allows the use of predefined extra options
    ///
    /// # Example
    ///
    /// ```
    /// let mut cli = CLI::new();
    /// let kwargs = CommandOptionKwargsBuilder::new()
    /// .set_deprecated()
    /// .set_required();
    /// cli.add_option("-v", "--verbose", "Enable verbose mode", &kwargs);
    /// ```
    fn add_option_kwargs(
        &mut self,
        short: &str,
        long: &str,
        description: &str,
        kwargs: &'a CommandOptionKwargs,
    ) -> &mut Self {
        // Index check if already added
        assert!(
            self.lookup_mut().get(short).is_none() && self.lookup_mut().get(long).is_none(),
            "short and/or long options already exist"
        );
        let arg = CommandOptionBuilder::new()
            .set_short(short)
            .set_long(long)
            .set_description(description)
            .set_kwargs(kwargs)
            .build();
        self.add(arg);
        self
    }
}
impl<'a, T: _CLICommand<'a>> CLICommand<'a> for T {}

#[derive(Default, Debug)]
pub(crate) struct Command<'a> {
    pub(crate) name: &'a str,
    pub(crate) commands: Vec<Command<'a>>,
    pub(crate) options: Vec<CommandOption<'a>>,
    pub(crate) lookup: HashSet<String>,
}
impl<'a> _CLICommand<'a> for Command<'a> {
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
impl<'a> Command<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use super::*;

    #[derive(Debug, Default)]
    pub(crate) struct MockCommand<'a> {
        pub(crate) commands: Vec<Command<'a>>,
        pub(crate) options: Vec<CommandOption<'a>>,
        pub(crate) lookup: HashSet<String>,
    }
    impl<'a> _CLICommand<'a> for MockCommand<'a> {
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
}

#[cfg(test)]
mod add_option_tests {
    use super::{mock::MockCommand, *};

    #[test]
    fn test_add_unique_option() {
        let mut command: MockCommand = MockCommand::default();
        command.add_option("-a", "--apple", "Description for Apple option");

        assert_eq!(
            command.options,
            [CommandOption {
                short: "-a".to_string(),
                long: "--apple".to_string(),
                description: "Description for Apple option".to_string(),
                kwargs: None,
            }],
        );
    }

    #[test]
    #[should_panic(expected = "short and/or long options already exist")]
    fn test_add_option_with_conflicting_short_name() {
        let mut command: MockCommand = MockCommand::default();
        command.add_option("-a", "--apple", "mock");
        command.add_option("-a", "--apricot", "mock");
    }

    #[test]
    #[should_panic(expected = "short and/or long options already exist")]
    fn test_add_option_with_conflicting_long_name() {
        // Arrange
        let mut command: MockCommand = MockCommand::default();
        command.add_option("-a", "--apple", "mock");
        // dbg!(command.lookup);
        command.add_option("-ab", "--apple", "mock");
    }

    #[test]
    #[should_panic(expected = "short and/or long options already exist")]
    fn test_add_option_with_conflicting_long_and_short_name() {
        // Arrange
        let mut command: MockCommand = MockCommand::default();
        command.add_option("-a", "--apple", "mock");
        // dbg!(command.lookup);
        command.add_option("-ab", "--apple", "mock");
    }
}

#[cfg(test)]
mod add_option_kwargs_tests {
    use super::{mock::MockCommand, *};
    use crate::cli_toolkit::option::CommandOptionKwargsBuilder;

    #[test]
    fn test_add_unique_option() {
        let mut command: MockCommand = MockCommand::default();
        let mut builder = CommandOptionKwargsBuilder::new();
        let kwargs = builder.set_deprecated().build();
        command.add_option_kwargs("-a", "--apple", "Description for Apple option", &kwargs);

        assert_eq!(
            command.options,
            [CommandOption {
                short: "-a".to_string(),
                long: "--apple".to_string(),
                description: "Description for Apple option".to_string(),
                kwargs: Some(&kwargs),
            }],
        );
    }

    #[test]
    #[should_panic(expected = "short and/or long options already exist")]
    fn test_add_option_kwargs_with_conflicting_short_name() {
        let mut command: MockCommand = MockCommand::default();
        let mut builder = CommandOptionKwargsBuilder::new();
        let kwargs = builder.set_deprecated().build();
        command.add_option_kwargs("-a", "--apple", "mock", &kwargs);
        command.add_option_kwargs("-a", "--apricot", "mock", &kwargs);
    }

    #[test]
    #[should_panic(expected = "short and/or long options already exist")]
    fn test_add_option_kwargs_with_conflicting_long_name() {
        let mut command: MockCommand = MockCommand::default();
        let mut builder = CommandOptionKwargsBuilder::new();
        let kwargs = builder.set_deprecated().build();
        command.add_option_kwargs("-a", "--apple", "mock", &kwargs);
        command.add_option_kwargs("-ab", "--apple", "mock", &kwargs);
    }

    #[test]
    #[should_panic(expected = "short and/or long options already exist")]
    fn test_add_option_kwargs_with_conflicting_long_and_short_name() {
        let mut command: MockCommand = MockCommand::default();
        let mut builder = CommandOptionKwargsBuilder::new();
        let kwargs = builder.set_deprecated().build();
        command.add_option_kwargs("-a", "--apple", "mock", &kwargs);
        command.add_option_kwargs("-ab", "--apple", "mock", &kwargs);
    }
}
