use crate::command::Action;
use crate::command::Command;
use crate::command::Error;

pub struct QuitCommand {}

impl Command for QuitCommand {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        Ok(vec![Action::Quit])
    }
}
