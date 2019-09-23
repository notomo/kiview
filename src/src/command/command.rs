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
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum Layout {
    Tab,
    Vertical,
    Unknown,
}

impl Layout {
    pub fn action(&self) -> String {
        match self {
            Layout::Tab => "tab_open",
            Layout::Vertical => "vertical_open",
            Layout::Unknown => "open",
        }
        .to_string()
    }
}

impl From<&str> for Layout {
    fn from(s: &str) -> Self {
        match s {
            "tab" => Layout::Tab,
            "vertical" => Layout::Vertical,
            _ => Layout::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum CommandOption {
    Layout { value: Layout },
    Path { value: String },
    Quit,
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
            ["go"] => CommandName::Go,
            ["new"] => CommandName::New,
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
            ["quit"] => CommandOption::Quit,
            ["path", path] => CommandOption::Path {
                value: path.to_string(),
            },
            _ => CommandOption::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct CommandOptions {
    pub layout: Option<Layout>,
    pub quit: bool,
    pub path: Option<String>,
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

        let path: Option<String> = options
            .iter()
            .map(|opt| match &opt {
                CommandOption::Path { value } => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<Option<String>>>()
            .get(0)
            .and_then(|path| path.clone());

        let quit = options.iter().any(|opt| match &opt {
            CommandOption::Quit => true,
            _ => false,
        });

        CommandOptions {
            layout: layout,
            quit: quit,
            path: path,
        }
    }
}
