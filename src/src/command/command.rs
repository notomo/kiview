use crate::command;
use crate::command::Action;
use crate::command::Current;
use crate::command::Error;
use crate::repository::Dispatcher;
use serde_derive::Serialize;

pub type CommandResult = Result<Vec<Action>, Error>;

pub trait Command {
    fn actions(&self) -> CommandResult;
}

pub struct SimpleCommand {
    pub action: Action,
}

impl Command for SimpleCommand {
    fn actions(&self) -> CommandResult {
        Ok(vec![self.action.clone()])
    }
}

#[derive(Debug)]
pub enum CommandName {
    Quit,
    Parent,
    Child,
    Go,
    New,
    Remove,
    Copy,
    Cut,
    Paste,
    Rename,
    MultipleRename,
    ToggleTree,
    ToggleSelection,
    ToggleAllSelection,
    Back,
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

impl Split {
    pub fn leaf_node_action(&self, paths: Vec<String>) -> Vec<Action> {
        if paths.len() == 0 {
            return vec![];
        }
        match self.name {
            SplitName::Unknown | SplitName::No => vec![],
            _ => vec![Action::OpenLeaves {
                paths: paths,
                split_name: self.name,
                mod_name: self.mod_name,
            }],
        }
    }
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
    Open { value: Split },
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
            ["open", split] => CommandOption::Open {
                value: Split::from(*split),
            },
            _ => CommandOption::Unknown,
        }
    }
}

pub fn parse_command_actions(arg: &str, current: &Current) -> CommandResult {
    let command_name = CommandName::from(arg);

    let opts: Vec<CommandOption> = arg
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

    let dispatcher = Dispatcher {};
    let path_repository = dispatcher.path_repository();

    let command = match &command_name {
        CommandName::Quit => box SimpleCommand {
            action: Action::Quit,
        } as Box<dyn Command>,
        CommandName::Parent => box command::ParentCommand {
            current: current,
            repository: path_repository,
        } as Box<dyn Command>,
        CommandName::Child => box command::ChildCommand {
            current: current,
            repository: path_repository,
            opts: opts.into(),
        } as Box<dyn Command>,
        CommandName::Go => box command::GoCommand {
            current: current,
            repository: path_repository,
            opts: opts.into(),
        } as Box<dyn Command>,
        CommandName::New => box command::NewCommand {
            current: current,
            repository: path_repository,
            opts: opts.into(),
        } as Box<dyn Command>,
        CommandName::Remove => box command::RemoveCommand {
            current: current,
            repository: path_repository,
            opts: opts.into(),
        } as Box<dyn Command>,
        CommandName::Copy => box command::CopyCommand { current: current } as Box<dyn Command>,
        CommandName::Cut => box command::CutCommand { current: current } as Box<dyn Command>,
        CommandName::Paste => box command::PasteCommand {
            current: current,
            repository: path_repository,
        } as Box<dyn Command>,
        CommandName::Rename => box command::RenameCommand {
            current: current,
            repository: path_repository,
            opts: opts.into(),
        } as Box<dyn Command>,
        CommandName::MultipleRename => box command::MultipleRenameCommand {
            current: current,
            repository: path_repository,
        } as Box<dyn Command>,
        CommandName::ToggleTree => box command::ToggleTreeCommand {
            current: current,
            repository: path_repository,
        } as Box<dyn Command>,
        CommandName::ToggleSelection => {
            box command::ToggleSelectionCommand { current: current } as Box<dyn Command>
        }
        CommandName::ToggleAllSelection => box SimpleCommand {
            action: Action::ToggleAllSelection,
        } as Box<dyn Command>,
        CommandName::Back => box SimpleCommand {
            action: Action::BackHistory,
        } as Box<dyn Command>,
        CommandName::Unknown => {
            box command::UnknownCommand { command_name: &arg } as Box<dyn Command>
        }
    };

    command.actions()
}
