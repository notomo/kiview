use crate::command::Action;
use crate::command::Command;

pub struct QuitCommand {}

impl Command for QuitCommand {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        Ok(vec![Action::Quit])
    }
}
