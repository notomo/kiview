use super::action::RegisteredItem;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;
use crate::repository::PathRepository;

pub struct CopyCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
}

impl<'a> Command for CopyCommand<'a> {
    fn actions(&self) -> CommandResult {
        Ok(vec![
            Action::Copy {
                items: self.current.target_items(&self.repository)?,
            },
            Action::UnselectAll,
        ])
    }
}

pub struct CutCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> CommandResult {
        Ok(vec![
            Action::Cut {
                items: self.current.target_items(&self.repository)?,
            },
            Action::UnselectAll,
        ])
    }
}

impl<'a> Current<'a> {
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
