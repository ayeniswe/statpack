use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
pub struct CommandOptionKwargs {
    pub(super) deprecated: bool,
    pub(super) required: bool,
    pub(super) nargs: Option<usize>,
    pub(super) default: Option<CommandOptionType>,
    pub(super) flag: Option<bool>,
    pub(super) choices: Option<Vec<CommandOptionType>>,
}
impl CommandOptionKwargs {
    fn new(
        deprecated: bool,
        required: bool,
        nargs: Option<usize>,
        default: Option<CommandOptionType>,
        flag: Option<bool>,
        choices: Option<Vec<CommandOptionType>>,
    ) -> Self {
        Self {
            deprecated,
            required,
            nargs,
            default,
            flag,
            choices,
        }
    }
}

#[derive(Default)]
pub struct CommandOptionKwargsBuilder {
    deprecated: bool,
    required: bool,
    nargs: Option<usize>,
    default: Option<CommandOptionType>,
    flag: Option<bool>,
    choices: Option<Vec<CommandOptionType>>,
}
impl CommandOptionKwargsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_deprecated(&mut self) -> &mut Self {
        self.deprecated = true;
        self
    }
    pub fn set_required(&mut self) -> &mut Self {
        self.required = true;
        self
    }
    pub fn set_nargs(&mut self, nargs: usize) -> &mut Self {
        self.nargs = Some(nargs);
        self
    }
    pub fn set_default(&mut self, default: CommandOptionType) -> &mut Self {
        self.default = Some(default);
        self
    }
    pub fn set_flag(&mut self, flag: bool) -> &mut Self {
        self.flag = Some(flag);
        self
    }
    pub fn set_choices(&mut self, choices: Vec<CommandOptionType>) -> &mut Self {
        self.choices = Some(choices);
        self
    }
    pub fn build(&self) -> CommandOptionKwargs {
        CommandOptionKwargs::new(
            self.deprecated,
            self.required,
            self.nargs,
            self.default.clone(),
            self.flag,
            self.choices.clone(),
        )
    }
}

#[derive(Debug, Clone, Default)]
pub enum CommandOptionType {
    #[default]
    None,
    Text(String),
    Int(i32),
    Float(f64),
}

#[derive(Debug)]
pub(super) struct CommandOption {
    pub(super) short: String,
    pub(super) long: String,
    description: String,
    kwargs: CommandOptionKwargs,
}
impl CommandOption {
    pub(super) fn new(
        short: String,
        long: String,
        description: String,
        kwargs: CommandOptionKwargs,
    ) -> Self {
        Self {
            short,
            long,
            description,
            kwargs,
        }
    }
}

#[derive(Default)]
pub(super) struct CommandOptionBuilder {
    short: String,
    long: String,
    description: String,
    kwargs: CommandOptionKwargs,
}
impl CommandOptionBuilder {
    pub(super) fn new() -> Self {
        Self::default()
    }
    /// Generate a short option flag
    pub(super) fn gen_short(&mut self, option: &str, lookup_table: &HashSet<String>) -> &mut Self {
        let mut short = String::from("-");
        for char in option.chars() {
            short.push(char);
            if lookup_table.get(short.as_str()).is_none() {
                break;
            }
        }
        self.short = short;
        self
    }
    /// Generate a long option flag
    pub(super) fn gen_long(&mut self, option: &str) -> &mut Self {
        self.long = format!("--{}", option);
        self
    }
    pub(super) fn set_short(&mut self, short: &str) -> &mut Self {
        self.short = short.to_string();
        self
    }
    pub(super) fn set_long(&mut self, long: &str) -> &mut Self {
        self.long = long.to_string();
        self
    }
    pub(super) fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }
    pub(super) fn set_kwargs(&mut self, kwargs: CommandOptionKwargs) -> &mut Self {
        self.kwargs = kwargs;
        self
    }
    pub(super) fn build(&self) -> CommandOption {
        CommandOption::new(
            self.short.clone(),
            self.long.clone(),
            self.description.clone(),
            self.kwargs.clone(),
        )
    }
}
