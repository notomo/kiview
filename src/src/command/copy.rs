use super::action::RegisteredTarget;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;

pub struct CopyCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for CopyCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let targets = self
            .current
            .targets()
            .map(|target| RegisteredTarget {
                path: target.to_string(),
            })
            .collect();

        Ok(vec![Action::Copy { targets: targets }])
    }
}
