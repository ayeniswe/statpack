use std::collections::HashSet;

/// Represents additional configuration options for a command-line option.
///
/// The `CommandOptionKwargs` struct is used to specify various optional parameters
/// for a command-line option, such as whether it is deprecated, required, or if it
/// has a default value or a set of valid choices.
#[derive(Default, Debug, Clone)]
pub struct CommandOptionKwargs<'a> {
    pub(super) deprecated: bool,
    pub(super) required: bool,
    pub(super) nargs: Option<usize>,
    pub(super) default: Option<&'a CommandOptionType>,
    pub(super) flag: Option<bool>,
    pub(super) choices: Option<&'a Vec<&'a CommandOptionType>>,
    // pub(super) prompt: Option<&'a str>,
    pub(super) confirmation: Option<&'a str>,
}
impl<'a> CommandOptionKwargs<'a> {
    fn new(
        deprecated: bool,
        required: bool,
        nargs: Option<usize>,
        default: Option<&'a CommandOptionType>,
        flag: Option<bool>,
        choices: Option<&'a Vec<&'a CommandOptionType>>,
        // prompt: Option<&'a str>,
        confirmation: Option<&'a str>,
    ) -> Self {
        Self {
            deprecated,
            required,
            nargs,
            default,
            flag,
            choices,
            // prompt,
            confirmation,
        }
    }
}

/// A builder for creating a `CommandOptionKwargs` instance
#[derive(Default)]
pub struct CommandOptionKwargsBuilder<'a> {
    deprecated: bool,
    required: bool,
    nargs: Option<usize>,
    default: Option<&'a CommandOptionType>,
    flag: Option<bool>,
    choices: Option<&'a Vec<&'a CommandOptionType>>,
    // prompt: Option<&'a str>,
    confirmation: Option<&'a str>,
}
impl<'a> CommandOptionKwargsBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    /// Indicates if the command-line option is deprecated.
    pub fn set_deprecated(&mut self) -> &mut Self {
        self.deprecated = true;
        self
    }
    /// Specifies if the command-line option is required.
    pub fn set_required(&mut self) -> &mut Self {
        self.required = true;
        self
    }
    /// Specifies the number of arguments that the command-line option takes.
    pub fn set_nargs(&mut self, nargs: usize) -> &mut Self {
        self.nargs = Some(nargs);
        self
    }
    /// Provides a default value for the command-line option.
    pub fn set_default(&mut self, default: &'a CommandOptionType) -> &mut Self {
        self.default = Some(default);
        self
    }
    /// Indicates if the command-line option is a flag (boolean) option.
    pub fn set_flag(&mut self, flag: bool) -> &mut Self {
        self.flag = Some(flag);
        self
    }
    /// Specifies a set of valid choices for the command-line option.
    pub fn set_choices(&mut self, choices: &'a mut Vec<&'a CommandOptionType>) -> &mut Self {
        // Default must be in choices if specified
        if let Some(default) = self.default {
            choices.push(default);
        }
        self.choices = Some(choices);
        self
    }
    // /// Provides a prompt message for interactive user input.
    // pub fn set_prompt(&mut self, prompt: &'a str) -> &mut Self {
    //     self.prompt = Some(prompt);
    //     self
    // }
    /// Indicates if the command-line option requires confirmation.
    pub fn set_confirmation(&mut self, confirmation: &'a str) -> &mut Self {
        self.confirmation = Some(confirmation);
        self
    }
    pub fn build(&self) -> CommandOptionKwargs {
        CommandOptionKwargs::new(
            self.deprecated,
            self.required,
            self.nargs,
            self.default,
            self.flag,
            self.choices,
            // self.prompt,
            self.confirmation,
        )
    }
}

/// Represents the type of a command-line option.
///
/// The `CommandOptionType` enum is used to define the type of value that a command-line option can hold.
/// It supports various types such as `None`, `Text`, `Int`, `Float`, `File`.
#[derive(Debug, Clone)]
pub enum CommandOptionType {
    Text(String),
    File(String),
    Int(i32),
    Float(f64),
}

/// Represents a command-line option with associated metadata and optional parameters.
///
/// The `CommandOption` struct is used to define a command-line option with its short and long
/// versions, a description, and optional additional parameters encapsulated in `CommandOptionKwargs`.
///
#[derive(Debug)]
pub struct CommandOption<'a> {
    pub(super) short: String,
    pub(super) long: String,
    description: String,
    kwargs: Option<&'a CommandOptionKwargs<'a>>,
}
impl<'a> CommandOption<'a> {
    pub(super) fn new(
        short: String,
        long: String,
        description: String,
        kwargs: Option<&'a CommandOptionKwargs>,
    ) -> Self {
        Self {
            short,
            long,
            description,
            kwargs,
        }
    }
}

/// A builder for creating a `CommandOption` instance
#[derive(Default)]
pub(super) struct CommandOptionBuilder<'a> {
    short: String,
    long: String,
    description: String,
    kwargs: Option<&'a CommandOptionKwargs<'a>>,
}
impl<'a> CommandOptionBuilder<'a> {
    pub(super) fn new() -> Self {
        Self::default()
    }
    /// A `-` short option flag
    pub(super) fn gen_short(&mut self, option: &str, lookup_table: &HashSet<String>) -> &mut Self {
        assert!(option.len() != 0, "option can not be empty");

        let mut short = String::from("-");
        let validate_gen = |string: String| {
            if lookup_table.get(string.as_str()).is_some() {
                return String::new();
            }
            string
        };

        // First generation attempt
        for char in option.chars() {
            short.push(char);
            if lookup_table.get(short.as_str()).is_none() {
                break;
            }
        }
        short = validate_gen(short);

        // Second generation attempt
        if short.is_empty() {
            short = String::from("-");

            let option_chars = option.chars();
            for char in option_chars.step_by(2) {
                short.push(char);
                if lookup_table.get(short.as_str()).is_none() {
                    break;
                }
            }
        }
        short = validate_gen(short);

        self.short = short;
        self
    }
    /// A `--` long option flag
    pub(super) fn gen_long(&mut self, option: &str) -> &mut Self {
        self.long = format!("--{}", option);
        self
    }
    /// A specified short option flag
    ///
    /// Useful for unconvential flags rather than `-` or `--`
    pub(super) fn set_short(&mut self, short: &str) -> &mut Self {
        self.short = short.to_string();
        self
    }
    /// A specified long option flag
    ///
    /// Useful for unconvential flags rather than `-` or `--`
    pub(super) fn set_long(&mut self, long: &str) -> &mut Self {
        self.long = long.to_string();
        self
    }
    pub(super) fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }
    /// Additional configuration options (kwargs)
    pub(super) fn set_kwargs(&mut self, kwargs: &'a CommandOptionKwargs) -> &mut Self {
        self.kwargs = Some(kwargs);
        self
    }
    pub(super) fn build(&self) -> CommandOption<'a> {
        CommandOption::new(
            self.short.clone(),
            self.long.clone(),
            self.description.clone(),
            self.kwargs,
        )
    }
}

#[cfg(test)]
mod gen_short_tests {
    use super::*;

    #[test]
    fn test_gen_short_no_conflict() {
        let lookup_table = HashSet::new();
        let option = CommandOptionBuilder::new()
            .gen_short("apple", &lookup_table)
            .build();

        assert_eq!(option.short, "-a");
    }

    #[test]
    fn test_gen_short_with_conflicts() {
        let mut lookup_table = HashSet::new();
        lookup_table.insert("-a".to_string());
        lookup_table.insert("-ap".to_string());
        let option = CommandOptionBuilder::new()
            .gen_short("apple", &lookup_table)
            .build();

        assert_eq!(option.short, "-app");
    }

    #[test]
    #[should_panic(expected = "option can not be empty")]
    fn test_gen_short_empty_option() {
        let lookup_table = HashSet::new();
        let option = CommandOptionBuilder::new()
            .gen_short("", &lookup_table)
            .build();

        assert_eq!(option.short, "");
    }

    #[test]
    fn test_gen_short_second_gen() {
        let mut lookup_table = HashSet::new();
        lookup_table.insert("-a".to_string());
        lookup_table.insert("-ap".to_string());
        lookup_table.insert("-app".to_string());
        lookup_table.insert("-appl".to_string());
        lookup_table.insert("-apple".to_string());

        let option = CommandOptionBuilder::new()
            .gen_short("apple", &lookup_table)
            .build();

        assert_eq!(option.short, "-ape");
    }

    #[test]
    fn test_gen_short_max_exhaustive() {
        let mut lookup_table = HashSet::new();
        lookup_table.insert("-a".to_string());
        lookup_table.insert("-ap".to_string());
        lookup_table.insert("-app".to_string());
        lookup_table.insert("-appl".to_string());
        lookup_table.insert("-apple".to_string());
        lookup_table.insert("-ape".to_string());
        let option = CommandOptionBuilder::new()
            .gen_short("apple", &lookup_table)
            .build();

        assert_eq!(option.short, "");
    }
}

#[cfg(test)]
mod gen_long_tests {
    use super::*;

    #[test]
    fn test_gen_long() {
        let option = CommandOptionBuilder::new().gen_long("apple").build();

        assert_eq!(option.long, "--apple");
    }
}
