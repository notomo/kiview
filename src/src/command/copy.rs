use super::current::RegisteredTarget;
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
            .iter()
            .map(|target| RegisteredTarget {
                path: target.to_string(),
                name: None,
            })
            .collect();

        Ok(vec![Action::Copy { targets: targets }])
    }
}
