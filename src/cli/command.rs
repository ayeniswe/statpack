mod internal {
    use super::{Command, CommandOption};
    pub(super) trait CommandInternal {
        /// Adds an option to the list and updates the lookup table for indexing and preventing collisions.
        /// This ensures that the option is unique and avoids duplicate entries.
        fn add(&mut self, arg: CommandOption);
    }
    impl<T: Command> CommandInternal for T {
        fn add(&mut self, arg: CommandOption) {
            let short = arg.short.clone();
            let long = arg.long.clone();
            self.lookup_mut().insert(short);
            self.lookup_mut().insert(long);
            self.options_mut().push(arg);
        }
    }
}

use self::internal::CommandInternal;
use super::option::{CommandOption, CommandOptionBuilder, CommandOptionKwargs, CommandOptionType};
use super::parser::Parser;
use std::collections::HashSet;

pub trait Command: CommandInternal + Parser {
    /// Returns a immutable reference to the lookup table, or `None` if it is empty
    fn lookup(&self) -> &HashSet<String>;
    /// Returns a mutable reference to the lookup table, or `None` if it is empty
    fn lookup_mut(&mut self) -> &mut HashSet<String>;
    /// Returns a mutable reference to the options list, or `None` if it is empty
    fn options_mut(&mut self) -> &mut Vec<CommandOption>;
    /// Returns a immutable reference to the options list, or `None` if it is empty
    fn options(&self) -> &Vec<CommandOption>;
    /// Autogen a new command-line option and adds to option list
    ///
    /// This method simplifies the process of building options by automatically generating
    /// the short, and long flags of the option based on the provided option string.
    ///
    /// # Example
    ///
    /// ```
    /// let mut parser = Parser::new();
    /// parser.create_option("verbose", "Enable verbose mode");
    /// ```
    fn create_option(&mut self, option: &str, description: &str) -> &mut Self {
        let arg = CommandOptionBuilder::new()
            .gen_short(option, self.lookup())
            .gen_long(option)
            .set_description(description)
            .set_kwargs(CommandOptionKwargs::default())
            .build();
        self.add(arg);
        self
    }
    /// Similar to `create_option` but allows the use of predefined extra options
    ///
    /// # Example
    ///
    /// ```
    /// let mut parser = Parser::new();
    /// let kwargs = CommandOptionKwargsBuilder::new()
    /// .set_deprecated()
    /// .set_required();
    /// parser.create_option("verbose", "Enable verbose mode", kwargs);
    /// ```
    fn create_option_kwargs(
        &mut self,
        option: &str,
        description: &str,
        kwargs: CommandOptionKwargs,
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
    /// let mut parser = Parser::new();
    /// parser.add_option('-v', "--verbose", "Enable verbose mode");
    /// ```
    fn add_option(&mut self, short: &str, long: &str, description: &str) -> &mut Self {
        // Index check if already added
        assert!(
            self.lookup_mut().get(short).is_none() || self.lookup_mut().get(long).is_none(),
            "[Error]: short and/or long options already exist"
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
    /// let mut parser = Parser::new();
    /// let kwargs = CommandOptionKwargsBuilder::new()
    /// .set_deprecated()
    /// .set_required();
    /// parser.add_option('-v', "--verbose", "Enable verbose mode", kwargs);
    /// ```
    fn add_option_kwargs(
        &mut self,
        short: &str,
        long: &str,
        description: &str,
        kwargs: CommandOptionKwargs,
    ) -> &mut Self {
        // Index check if already added
        assert!(
            self.lookup_mut().get(short) != None || self.lookup_mut().get(long) != None,
            "[Error]: short and/or long options already exist"
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

#[derive(Debug, Default)]
pub struct MainCommand {
    options: Vec<CommandOption>,
    lookup: HashSet<String>,
}
impl Command for MainCommand {
    fn options(&self) -> &Vec<CommandOption> {
        &self.options
    }
    fn options_mut(&mut self) -> &mut Vec<CommandOption> {
        &mut self.options
    }
    fn lookup_mut(&mut self) -> &mut HashSet<String> {
        &mut self.lookup
    }
    fn lookup(&self) -> &HashSet<String> {
        &self.lookup
    }
}
