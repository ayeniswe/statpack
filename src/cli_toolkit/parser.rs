use super::{command::CLICommand, option::CommandOption};
use crate::utils::search::bisect_search_str_key;

/// The `Parser` trait provides methods for parsing command-line commands associated with a specific command implementation.
/// Implementations of this trait are responsible for allowing the command to handle command-line arguments efficiently.
pub trait Parser {
    /// Searches for the specified option within the internal options list.
    ///
    /// ## Returns
    ///
    /// A vector of strings containing the long names of options that match the given option string.
    ///
    /// ## Examples
    ///
    /// ```
    /// let mut command = MockCommand::default();
    /// command.create_option("verbose", "Enable verbose mode");
    /// command.create_option("version", "Display version information");
    ///
    /// let matches = command.search_options("-v");
    /// assert_eq!(matches, vec!["--verbose", "--version"]);
    /// ```
    fn search_options(&self, option: &str) -> Vec<String>;
    /// Sort internal options list
    fn sort(&mut self);
}
impl<'a, T: CLICommand<'a>> Parser for T {
    fn search_options(&self, option: &str) -> Vec<String> {
        let mut find = vec![];
        let options = self.options();

        // Search range if multiple options
        let left_index;
        let right_index;
        let key_extractor = |item: &CommandOption, search_key: &str| {
            if search_key.starts_with("--") {
                item.long.clone()
            } else if search_key.starts_with("-") {
                item.short.clone()
            } else {
                "".to_string()
            }
        };
        left_index = bisect_search_str_key(self.options(), option, false, key_extractor);
        right_index = bisect_search_str_key(self.options(), option, true, key_extractor);

        if left_index != -1 && right_index != -1 {
            for index in left_index..right_index + 1 {
                find.push(options[index as usize].long.clone());
            }
        }
        find
    }
    fn sort(&mut self) {
        self.options_mut().sort_by(|a, b| a.long.cmp(&b.long));
    }
}

#[cfg(test)]
mod search_options_tests {
    use super::*;
    use crate::cli_toolkit::command::mock::MockCommand;

    #[test]
    fn test_search_options_single_option() {
        let mut command = MockCommand::default();
        command
            .add_option("-a", "--apple", "Description for Apple option")
            .add_option("-ap", "--apricot", "Description for Apple option")
            .sort();
        let result = command.search_options("--appl");

        assert_eq!(result, vec!["--apple"]);
    }

    #[test]
    fn test_search_options_multiple_options() {
        let mut command = MockCommand::default();
        command
            .add_option("-a", "--ab", "Description for Apple option")
            .add_option("-acd", "--acde", "Description for Apple option")
            .add_option("-ad", "--ade", "Description for Apple option")
            .add_option("-ab", "--abc", "Description for Apple option")
            .sort();
        let result = command.search_options("--ab");

        assert_eq!(result, vec!["--ab", "--abc"]);
    }

    #[test]
    fn test_search_options_option_not_found() {
        let mut command = MockCommand::default();
        command
            .add_option("-a", "--apple", "Description for Apple option")
            .add_option("-ab", "--applicable", "Description for Apple option")
            .sort();
        let result = command.search_options("--na");

        assert!(result.is_empty());
    }
}
