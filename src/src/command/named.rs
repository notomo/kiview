use crate::command::Command;
use crate::command::CommandName;

pub struct NamedCommand {
    pub name: CommandName,
}

impl Command for NamedCommand {
    fn actions(&self) -> serde_json::Value {
        json!([{
            "name": &self.name,
            "args": [],
            "options": {},
        }])
    }
}
