use super::{command::Command, option::CommandOption};
use crate::utils::search::bisect_search_str_key;
pub trait Parser {
    /// Searches for the specified option within the internal options list.
    /// ## Returns
    /// Returns a vector of matching options.
    fn search_options(&self, option: &str) -> Vec<String>;
    /// Sort command options list
    fn sort(&mut self);
}
impl<'a, T: Command<'a>> Parser for T {
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
