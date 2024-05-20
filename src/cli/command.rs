mod internal {
    use super::{Arg, Command};
    pub(super) trait CommandInternal {
        /// Adds an option to the list and updates the lookup table for indexing and preventing collisions.
        /// This ensures that the option is unique and avoids duplicate entries.
        fn add(&mut self, arg: Arg);
    }
    impl<T: Command> CommandInternal for T {
        fn add(&mut self, arg: Arg) {
            let short = arg.short.clone();
            let long = arg.long.clone();
            self.lookup_mut().insert(short);
            self.lookup_mut().insert(long);
            self.options_mut().push(arg);
        }
    }
}

use self::internal::CommandInternal;
use super::arg::{Arg, ArgBuilder};
use super::parser::Parser;
use std::collections::HashSet;
use std::fmt::Error;
pub trait Command: CommandInternal + Parser {
    /// Returns a mutable reference to the lookup table, or `None` if it is empty
    fn lookup_mut(&mut self) -> &mut HashSet<String>;
    /// Returns a mutable reference to the options list, or `None` if it is empty
    fn options_mut(&mut self) -> &mut Vec<Arg>;
    /// Returns a immutable reference to the options list, or `None` if it is empty
    fn options(&self) -> &Vec<Arg>;
    /// Autogen a new command-line option and adds to option list
    ///
    /// This method simplifies the process of building options by automatically generating
    /// the short, and long flags of the option based on the provided option string.
    ///
    /// # Example
    ///
    /// ```
    /// let mut parser = Parser::new();
    /// parser.create_option("verbose", "Enable verbose mode", false);
    /// ```
    fn create_option(&mut self, option: &str, description: &str, deprecated: bool) -> &mut Self {
        let arg = ArgBuilder::default()
            .set_long(option)
            .set_short(option)
            .set_description(description)
            .set_deprecated(deprecated)
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
    /// parser.add_option('-v', "--verbose", "Enable verbose mode", false);
    /// ```
    fn add_option(
        &mut self,
        short: &str,
        long: &str,
        description: &str,
        deprecated: bool,
    ) -> &mut Self {
        // Index check if already added
        assert!(
            self.lookup_mut().get(short) != None || self.lookup_mut().get(long) != None,
            "[Error]: short and/or long options already exist"
        );
        self.add(Arg::new(
            short.to_string(),
            long.to_string(),
            description.to_string(),
            deprecated,
        ));
        self
    }
}

#[derive(Debug, Default)]
pub struct MainCommand {
    options: Vec<Arg>,
    lookup: HashSet<String>,
}

impl Command for MainCommand {
    fn options(&self) -> &Vec<Arg> {
        &self.options
    }
    fn options_mut(&mut self) -> &mut Vec<Arg> {
        &mut self.options
    }
    fn lookup_mut(&mut self) -> &mut HashSet<String> {
        &mut self.lookup
    }
}
