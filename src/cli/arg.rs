use std::collections::HashSet;

#[derive(Default)]
pub struct OptionKwargs {
    pub(super) deprecated: bool,
    pub(super) required: bool,
    pub(super) nargs: Option<usize>,
    pub(super) default: Option<ArgDefault>,
    pub(super) flag: Option<bool>,
}
impl OptionKwargs {
    fn new(
        deprecated: bool,
        required: bool,
        nargs: Option<usize>,
        default: Option<ArgDefault>,
        flag: Option<bool>,
    ) -> Self {
        Self {
            deprecated,
            required,
            nargs,
            default,
            flag,
        }
    }
}

#[derive(Default)]
pub struct OptionKwargsBuilder {
    deprecated: bool,
    required: bool,
    nargs: Option<usize>,
    default: Option<ArgDefault>,
    flag: Option<bool>,
}
impl OptionKwargsBuilder {
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
    pub fn set_default(&mut self, default: ArgDefault) -> &mut Self {
        self.default = Some(default);
        self
    }
    pub fn set_flag(&mut self, flag: bool) -> &mut Self {
        self.flag = Some(flag);
        self
    }
    pub fn build(&self) -> OptionKwargs {
        OptionKwargs::new(
            self.deprecated,
            self.required,
            self.nargs,
            self.default.clone(),
            self.flag,
        )
    }
}

#[derive(Debug, Clone)]
pub(super) enum ArgDefault {
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
    default: Option<ArgDefault>,
    nargs: Option<usize>,
    flag: Option<bool>,
}
impl Arg {
    pub(super) fn new(
        short: String,
        long: String,
        description: String,
        deprecated: bool,
        required: bool,
        nargs: Option<usize>,
        default: Option<ArgDefault>,
        flag: Option<bool>,
    ) -> Self {
        Self {
            short,
            long,
            description,
            deprecated,
            required,
            nargs,
            default,
            flag,
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
    default: Option<ArgDefault>,
    nargs: Option<usize>,
    flag: Option<bool>,
}
impl ArgBuilder {
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
    pub(super) fn set_kwargs(&mut self, kwargs: OptionKwargs) -> &mut Self {
        self.deprecated = kwargs.deprecated;
        self.default = kwargs.default;
        self.nargs = kwargs.nargs;
        self.required = kwargs.required;
        self.flag = kwargs.flag;
        self
    }
    pub(super) fn build(&self) -> Arg {
        Arg::new(
            self.short.clone(),
            self.long.clone(),
            self.description.clone(),
            self.deprecated,
            self.required,
            self.nargs,
            self.default.clone(),
            self.flag,
        )
    }
}
