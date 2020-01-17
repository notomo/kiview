use crate::command::Action;
use crate::command::Error;
use serde_derive::Serialize;

pub trait Command {
    fn actions(&self) -> Result<Vec<Action>, Error>;
}

#[derive(Debug, Serialize)]
pub enum CommandName {
    #[serde(rename = "quit")]
    Quit,
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
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
    #[serde(rename = "multiple_rename")]
    MultipleRename,
    #[serde(rename = "toggle_tree")]
    ToggleTree,
    #[serde(rename = "toggle_selection")]
    ToggleSelection,
    #[serde(rename = "toggle_all_selection")]
    ToggleAllSelection,
    #[serde(rename = "back")]
    Back,
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
            ["multiple_rename"] => CommandName::MultipleRename,
            ["toggle_tree"] => CommandName::ToggleTree,
            ["toggle_selection"] => CommandName::ToggleSelection,
            ["toggle_all_selection"] => CommandName::ToggleAllSelection,
            ["back"] => CommandName::Back,
            [] => CommandName::Go,
            _ => CommandName::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Layout {
    Tab,
    Vertical,
    Horizontal,
    Open,
}

impl Layout {
    pub fn leaf_node_action(&self, paths: Vec<String>) -> Action {
        match self {
            Layout::Tab => Action::TabOpen { paths: paths },
            Layout::Vertical => Action::VerticalOpen { paths: paths },
            Layout::Horizontal => Action::HorizontalOpen { paths: paths },
            Layout::Open => Action::Open { paths: paths },
        }
    }
}

impl From<&str> for Layout {
    fn from(s: &str) -> Self {
        match s {
            "tab" => Layout::Tab,
            "vertical" => Layout::Vertical,
            "horizontal" => Layout::Horizontal,
            _ => Layout::Open,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Split {
    pub name: SplitName,
    pub mod_name: SplitModName,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SplitName {
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "unknown")]
    Unknown,
}

impl From<Layout> for SplitName {
    fn from(l: Layout) -> Self {
        match l {
            Layout::Tab => SplitName::Tab,
            Layout::Vertical => SplitName::Vertical,
            Layout::Horizontal => SplitName::Horizontal,
            Layout::Open => SplitName::No,
        }
    }
}

impl From<&str> for SplitName {
    fn from(s: &str) -> Self {
        match s {
            "tab" => SplitName::Tab,
            "vertical" => SplitName::Vertical,
            "horizontal" => SplitName::Horizontal,
            "no" => SplitName::No,
            _ => SplitName::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SplitModName {
    #[serde(rename = "leftabove")]
    LeftAbove,
    #[serde(rename = "rightbelow")]
    RightBelow,
    #[serde(rename = "")]
    No,
    #[serde(rename = "unknown")]
    Unknown,
}

impl From<&str> for SplitModName {
    fn from(s: &str) -> Self {
        match s {
            "leftabove" => SplitModName::LeftAbove,
            "rightbelow" => SplitModName::RightBelow,
            _ => SplitModName::Unknown,
        }
    }
}

impl From<&str> for Split {
    fn from(s: &str) -> Self {
        let names: Vec<_> = s.split(":").collect();
        match &names[..] {
            [name, mod_name] => Split {
                name: SplitName::from(*name),
                mod_name: SplitModName::from(*mod_name),
            },
            [""] => Split {
                name: SplitName::Vertical,
                mod_name: SplitModName::LeftAbove,
            },
            [name] => Split {
                name: SplitName::from(*name),
                mod_name: SplitModName::No,
            },
            _ => Split {
                name: SplitName::Unknown,
                mod_name: SplitModName::Unknown,
            },
        }
    }
}

#[derive(Debug)]
pub enum CommandOption {
    Layout { value: Layout },
    Path { value: String },
    Paths { value: String },
    Quit,
    NoConfirm,
    Create,
    Back,
    Split { value: Split },
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
            ["create"] => CommandOption::Create,
            ["back"] => CommandOption::Back,
            ["path", path] => CommandOption::Path {
                value: path.to_string(),
            },
            ["paths", path] => CommandOption::Paths {
                value: path.to_string(),
            },
            ["split", split] => CommandOption::Split {
                value: Split::from(*split),
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
    pub paths: Vec<String>,
    pub no_confirm: bool,
    pub create: bool,
    pub back: bool,
    pub split: Split,
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
            .filter_map(|opt| match &opt {
                CommandOption::Layout { value } => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<Layout>>()
            .get(0)
            .cloned()
            .unwrap_or(Layout::Open);

        let path: Option<String> = options
            .iter()
            .filter_map(|opt| match &opt {
                CommandOption::Path { value } => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<String>>()
            .get(0)
            .cloned();

        let paths: Vec<String> = options
            .iter()
            .filter_map(|opt| match &opt {
                CommandOption::Paths { value } => Some(value.clone()),
                _ => None,
            })
            .collect();

        let quit = options.iter().any(|opt| match &opt {
            CommandOption::Quit => true,
            _ => false,
        });

        let no_confirm = options.iter().any(|opt| match &opt {
            CommandOption::NoConfirm => true,
            _ => false,
        });

        let create = options.iter().any(|opt| match &opt {
            CommandOption::Create => true,
            _ => false,
        });

        let back = options.iter().any(|opt| match &opt {
            CommandOption::Back => true,
            _ => false,
        });

        let split: Split = options
            .iter()
            .filter_map(|opt| match &opt {
                CommandOption::Split { value } => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<Split>>()
            .get(0)
            .cloned()
            .unwrap_or(Split::from(""));

        CommandOptions {
            layout: layout,
            quit: quit,
            path: path,
            paths: paths,
            no_confirm: no_confirm,
            create: create,
            back: back,
            split: split,
        }
    }
}
