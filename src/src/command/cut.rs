use crate::command::Action;
use crate::command::Command;
use crate::command::Current;

pub struct CutCommand<'a> {
    pub current: Current<'a>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let paths = self
            .current
            .targets
            .iter()
            .map(|target| target.to_string())
            .collect();

        Ok(vec![Action::Cut { paths: paths }])
    }
}
