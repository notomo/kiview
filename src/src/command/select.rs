use crate::command::Action;
use crate::command::Command;
use crate::command::Current;

pub struct ToggleSelectionCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for ToggleSelectionCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let ids = self
            .current
            .targets
            .iter()
            .map(|target| target.id)
            .collect();

        Ok(vec![Action::ToggleSelection { ids: ids }])
    }
}
