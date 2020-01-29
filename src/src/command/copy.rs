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
        Ok(vec![
            Action::Copy {
                items: self.current.target_items(),
            },
            Action::UnselectAll,
        ])
    }
}

pub struct CutCommand<'a> {
    pub current: &'a Current<'a>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> CommandResult {
        Ok(vec![
            Action::Cut {
                items: self.current.target_items(),
            },
            Action::UnselectAll,
        ])
    }
}

impl<'a> Current<'a> {
    fn target_items(&self) -> Vec<RegisteredItem> {
        self.targets()
            .filter(|target| !target.is_parent_node)
            .map(|target| RegisteredItem {
                path: target.to_string(),
            })
            .collect()
    }
}
