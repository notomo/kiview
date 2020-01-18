use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct ParentCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
}

impl<'a> Command for ParentCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let parent_path = match self.repository.new_path(self.current.path).parent() {
            Some(parent_path) => parent_path,
            None => return Ok(vec![]),
        };

        let paths: Paths = self.repository.list(&parent_path)?.into();

        let mut actions = vec![paths.to_write_all_action()];

        if let Some(last_path_line_number) = paths.search(|p| p.path == self.current.path) {
            actions.push(Action::SetCursor {
                line_number: last_path_line_number,
            });
        }

        actions.extend(vec![
            Action::SetPath { path: parent_path },
            Action::AddHistory {
                path: self.current.path.to_string(),
                line_number: self.current.line_number,
                back: false,
            },
        ]);

        Ok(actions)
    }
}
