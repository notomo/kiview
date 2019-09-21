use crate::command::CommandName;

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
