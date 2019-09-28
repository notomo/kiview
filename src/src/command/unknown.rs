use crate::command::Action;
use crate::command::Command;

pub struct UnknownCommand {}

impl Command for UnknownCommand {
    fn actions(&self) -> Vec<Action> {
        vec![Action::Unknown]
    }
}
