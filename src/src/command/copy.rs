use super::action::RegisteredItem;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;
use crate::repository::PathRepository;

pub struct CopyCommand {
    pub current: Current,
    pub repository: Box<dyn PathRepository>,
}

impl Command for CopyCommand {
    fn actions(&self) -> CommandResult {
        Ok(vec![
            Action::Copy {
                items: self.current.target_items(&self.repository)?,
            },
            Action::UnselectAll,
        ])
    }
}

pub struct CutCommand {
    pub current: Current,
    pub repository: Box<dyn PathRepository>,
}

impl Command for CutCommand {
    fn actions(&self) -> CommandResult {
        Ok(vec![
            Action::Cut {
                items: self.current.target_items(&self.repository)?,
            },
            Action::UnselectAll,
        ])
    }
}

impl Current {
    fn target_items(
        &self,
        repository: &Box<dyn PathRepository>,
    ) -> Result<Vec<RegisteredItem>, Error> {
        self.targets()
            .filter(|target| !target.is_parent_node)
            .try_fold(vec![], |mut items, target| {
                let path = repository.path(&target.path).canonicalize()?;
                let item = RegisteredItem { path: path };
                items.push(item);
                Ok(items)
            })
    }
}
