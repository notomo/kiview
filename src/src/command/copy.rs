use super::action::RegisteredItem;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;

pub struct CopyCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for CopyCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let items = self
            .current
            .targets()
            .map(|target| RegisteredItem {
                path: target.to_string(),
            })
            .collect();

        Ok(vec![Action::Copy { items: items }])
    }
}

pub struct CutCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let items = self
            .current
            .targets()
            .map(|target| RegisteredItem {
                path: target.to_string(),
            })
            .collect();

        Ok(vec![Action::Cut { items: items }])
    }
}
