use crate::command::Action;
use crate::command::Command;

pub struct UnknownCommand {}

impl Command for UnknownCommand {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        Ok(vec![Action::Unknown])
    }
}
