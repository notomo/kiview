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

#[derive(Debug)]
pub enum LayoutName {
    Tab,
    Unknown,
}

impl From<&str> for LayoutName {
    fn from(s: &str) -> Self {
        match s {
            "tab" => LayoutName::Tab,
            _ => LayoutName::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum CommandOption {
    Layout(LayoutName),
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
            ["layout", layout] => CommandOption::Layout(LayoutName::from(*layout)),
            _ => CommandOption::Unknown,
        }
    }
}

pub struct NamedCommand {
    pub name: CommandName,
}

impl NamedCommand {
    pub fn actions(&self) -> serde_json::Value {
        json!([{
            "name": &self.name,
            "args": [],
            "options": {},
        }])
    }
}
