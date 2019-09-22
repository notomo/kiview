use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub enum CommandName {
    #[serde(rename = "quit")]
    Quit,
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum Layout {
    Tab,
    Unknown,
}

impl Layout {
    pub fn action(&self) -> String {
        match self {
            Layout::Tab => "tab_open",
            Layout::Unknown => "open",
        }
        .to_string()
    }
}

impl From<&str> for Layout {
    fn from(s: &str) -> Self {
        match s {
            "tab" => Layout::Tab,
            _ => Layout::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum CommandOption {
    Layout { value: Layout },
    Unknown,
}

impl From<&str> for CommandName {
    fn from(arg: &str) -> Self {
        let command_names: Vec<_> = arg
            .split_whitespace()
            .filter(|arg| !arg.starts_with("-"))
            .collect();

        match &command_names[..] {
            ["quit"] => CommandName::Quit,
            ["parent"] => CommandName::Parent,
            ["child"] => CommandName::Child,
            [] => CommandName::Create,
            _ => CommandName::Unknown,
        }
    }
}

impl From<&str> for CommandOption {
    fn from(arg: &str) -> Self {
        let key_value: Vec<_> = arg.split("=").collect();
        match &key_value[..] {
            ["layout", layout] => CommandOption::Layout {
                value: Layout::from(*layout),
            },
            _ => CommandOption::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct CommandOptions {
    pub layout: Option<Layout>,
}

impl CommandOptions {
    pub fn new(arg: &str) -> Self {
        let options: Vec<CommandOption> = arg
            .split_whitespace()
            .filter(|arg| arg.starts_with("-"))
            .map(|arg| {
                arg.chars()
                    .into_iter()
                    .skip(1)
                    .collect::<String>()
                    .as_str()
                    .into()
            })
            .collect();

        let layout: Option<Layout> = options
            .iter()
            .map(|opt| match &opt {
                CommandOption::Layout { value } => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<Option<Layout>>>()
            .get(0)
            .and_then(|layout| *layout);

        CommandOptions { layout: layout }
    }
}
