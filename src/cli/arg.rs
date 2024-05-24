use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum ArgType {
    Integer(i32),
    Float(f64),
    Text(String),
    // Add other types as needed
}

#[derive(Debug)]
pub(super) struct Arg {
    pub(super) short: String,
    pub(super) long: String,
    description: String,
    deprecated: bool,
    required: bool,
    default: Option<ArgType>,
}
impl Arg {
    pub(super) fn new(
        short: String,
        long: String,
        description: String,
        deprecated: bool,
        required: bool,
        default: Option<ArgType>,
    ) -> Self {
        Self {
            short,
            long,
            description,
            deprecated,
            required,
            default,
        }
    }
}

#[derive(Default, Clone)]
pub(super) struct ArgBuilder {
    short: String,
    long: String,
    description: String,
    deprecated: bool,
    required: bool,
    default: Option<ArgType>,
}
impl ArgBuilder {
    pub(super) fn new() -> Self {
        Self {
            short: Default::default(),
            long: Default::default(),
            description: Default::default(),
            deprecated: Default::default(),
            required: Default::default(),
            default: Default::default(),
        }
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
    pub(super) fn set_long(&mut self, option: &str) -> &mut Self {
        self.long = format!("--{}", option);
        self
    }
    pub(super) fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }
    pub(super) fn set_deprecated(&mut self, deprecated: bool) -> &mut Self {
        self.deprecated = deprecated;
        self
    }
    pub(super) fn set_required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }
    pub(super) fn set_default(&mut self, default: Option<ArgType>) -> &mut Self {
        self.default = default;
        self
    }
    pub(super) fn build(&self) -> Arg {
        Arg::new(
            self.short.clone(),
            self.long.clone(),
            self.description.clone(),
            self.deprecated,
            self.required,
            self.default.clone(),
        )
    }
}
