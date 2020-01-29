use super::action::RegisteredItem;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;

pub struct CopyCommand<'a> {
    pub current: &'a Current<'a>,
}

impl<'a> Command for CopyCommand<'a> {
    fn actions(&self) -> CommandResult {
        let items = self
            .current
            .targets()
            .filter(|target| !target.is_parent_node)
            .map(|target| RegisteredItem {
                path: target.to_string(),
            })
            .collect();

        Ok(vec![Action::Copy { items: items }, Action::UnselectAll])
    }
}

pub struct CutCommand<'a> {
    pub current: &'a Current<'a>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> CommandResult {
        let items = self
            .current
            .targets()
            .filter(|target| !target.is_parent_node)
            .map(|target| RegisteredItem {
                path: target.to_string(),
            })
            .collect();

        Ok(vec![Action::Cut { items: items }, Action::UnselectAll])
    }
}
