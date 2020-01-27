use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;

pub struct ToggleSelectionCommand<'a> {
    pub current: &'a Current<'a>,
}

impl<'a> Command for ToggleSelectionCommand<'a> {
    fn actions(&self) -> CommandResult {
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

pub struct SelectCommand<'a> {
    pub current: &'a Current<'a>,
}

impl<'a> Command for SelectCommand<'a> {
    fn actions(&self) -> CommandResult {
        let ids = self
            .current
            .targets
            .iter()
            .filter(|target| !target.is_parent_node)
            .map(|target| target.id)
            .collect();

        Ok(vec![Action::Select { ids: ids }])
    }
}

pub struct UnselectCommand<'a> {
    pub current: &'a Current<'a>,
}

impl<'a> Command for UnselectCommand<'a> {
    fn actions(&self) -> CommandResult {
        let ids = self
            .current
            .targets
            .iter()
            .filter(|target| !target.is_parent_node)
            .map(|target| target.id)
            .collect();

        Ok(vec![Action::Unselect { ids: ids }])
    }
}
