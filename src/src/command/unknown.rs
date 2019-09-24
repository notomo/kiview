use crate::command::Command;

pub struct UnknownCommand {}

impl Command for UnknownCommand {
    fn actions(&self) -> serde_json::Value {
        json!([])
    }
}
