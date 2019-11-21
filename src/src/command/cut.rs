use crate::command::Action;
use crate::command::Command;

pub struct CutCommand<'a> {
    pub targets: Vec<&'a str>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let paths = self
            .targets
            .iter()
            .map(|target| target.to_string())
            .collect();

        Ok(vec![Action::Cut { paths: paths }])
    }
}
