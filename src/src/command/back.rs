use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;

pub struct BackCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for BackCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        Ok(vec![Action::BackHistory {}])
    }
}
