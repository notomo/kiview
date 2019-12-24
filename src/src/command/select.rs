use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;

pub struct ToggleSelectionCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for ToggleSelectionCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let ids = self
            .current
            .targets
            .iter()
            .map(|target| target.id)
            .collect();

        Ok(vec![Action::ToggleSelection { ids: ids }])
    }
}

pub struct ToggleAllSelectionCommand {}

impl Command for ToggleAllSelectionCommand {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        Ok(vec![Action::ToggleAllSelection])
    }
}
