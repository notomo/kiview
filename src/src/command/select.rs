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
            .filter(|target| !target.is_parent_node)
            .map(|target| target.id)
            .collect();

        Ok(vec![Action::ToggleSelection { ids: ids }])
    }
}
