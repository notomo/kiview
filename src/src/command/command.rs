use crate::command::Action;
use serde_derive::Serialize;

pub trait Command {
    fn actions(&self) -> Vec<Action>;
}

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
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "rename")]
    Rename,
    #[serde(rename = "unknown")]
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
            ["remove"] => CommandName::Remove,
            ["copy"] => CommandName::Copy,
            ["cut"] => CommandName::Cut,
            ["paste"] => CommandName::Paste,
            ["rename"] => CommandName::Rename,
            [] => CommandName::Create,
            _ => CommandName::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Layout {
    Tab,
    Vertical,
    Open,
    Unknown,
}

impl Layout {
    pub fn action(&self, paths: Vec<String>) -> Action {
        match self {
            Layout::Tab => Action::TabOpen { paths: paths },
            Layout::Vertical => Action::VerticalOpen { paths: paths },
            Layout::Open => Action::Open { paths: paths },
            Layout::Unknown => Action::Unknown,
        }
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
    NoConfirm,
    Unknown,
}

impl From<&str> for CommandOption {
    fn from(arg: &str) -> Self {
        let key_value: Vec<_> = arg.split("=").collect();
        match &key_value[..] {
            ["layout", layout] => CommandOption::Layout {
                value: Layout::from(*layout),
            },
            ["quit"] => CommandOption::Quit,
            ["no-confirm"] => CommandOption::NoConfirm,
            ["path", path] => CommandOption::Path {
                value: path.to_string(),
            },
            _ => CommandOption::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct CommandOptions {
    pub layout: Layout,
    pub quit: bool,
    pub path: Option<String>,
    pub no_confirm: bool,
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

        let layout: Layout = options
            .iter()
            .map(|opt| match &opt {
                CommandOption::Layout { value } => Some(value.clone()),
                _ => None,
            })
            .filter(|opt| opt.is_some())
            .collect::<Vec<Option<Layout>>>()
            .get(0)
            .and_then(|layout| *layout)
            .unwrap_or(Layout::Open);

        let path: Option<String> = options
            .iter()
            .map(|opt| match &opt {
                CommandOption::Path { value } => Some(value.clone()),
                _ => None,
            })
            .filter(|opt| opt.is_some())
            .collect::<Vec<Option<String>>>()
            .get(0)
            .and_then(|path| path.clone());

        let quit = options.iter().any(|opt| match &opt {
            CommandOption::Quit => true,
            _ => false,
        });

        let no_confirm = options.iter().any(|opt| match &opt {
            CommandOption::NoConfirm => true,
            _ => false,
        });

        CommandOptions {
            layout: layout,
            quit: quit,
            path: path,
            no_confirm: no_confirm,
        }
    }
}
