use crate::command::Action;
use crate::command::Command;

pub struct QuitCommand {}

impl Command for QuitCommand {
    fn actions(&self) -> Vec<Action> {
        vec![Action::Quit {}]
    }
}
