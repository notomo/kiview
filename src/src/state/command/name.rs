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
    ClearClipboard,
    Paste,
    Rename,
    MultipleRename,
    ToggleTree,
    ToggleSelection,
    Select,
    SelectAll,
    Unselect,
    UnselectAll,
    ToggleAllSelection,
    Back,
    Unknown,
}

impl CommandName {
    pub fn all() -> Vec<String> {
        vec![
            "quit",
            "parent",
            "child",
            "go",
            "new",
            "remove",
            "copy",
            "cut",
            "clear_clipboard",
            "paste",
            "rename",
            "multiple_rename",
            "toggle_tree",
            "toggle_selection",
            "select",
            "select_all",
            "unselect",
            "unselect_all",
            "toggle_all_selection",
            "back",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn parse(arg: &str) -> (Self, usize) {
        let command_names: Vec<_> = arg
            .split_whitespace()
            .filter(|arg| !arg.starts_with("-"))
            .collect();

        let name = match &command_names[..] {
            ["quit"] => CommandName::Quit,
            ["parent"] => CommandName::Parent,
            ["child"] => CommandName::Child,
            ["go"] => CommandName::Go,
            ["new"] => CommandName::New,
            ["remove"] => CommandName::Remove,
            ["copy"] => CommandName::Copy,
            ["cut"] => CommandName::Cut,
            ["clear_clipboard"] => CommandName::ClearClipboard,
            ["paste"] => CommandName::Paste,
            ["rename"] => CommandName::Rename,
            ["multiple_rename"] => CommandName::MultipleRename,
            ["toggle_tree"] => CommandName::ToggleTree,
            ["toggle_selection"] => CommandName::ToggleSelection,
            ["select"] => CommandName::Select,
            ["select_all"] => CommandName::SelectAll,
            ["unselect"] => CommandName::Unselect,
            ["unselect_all"] => CommandName::UnselectAll,
            ["toggle_all_selection"] => CommandName::ToggleAllSelection,
            ["back"] => CommandName::Back,
            [] => CommandName::Go,
            _ => CommandName::Unknown,
        };
        (name, command_names.len())
    }
}

impl From<&str> for CommandName {
    fn from(arg: &str) -> Self {
        let (name, _) = CommandName::parse(arg);
        name
    }
}
