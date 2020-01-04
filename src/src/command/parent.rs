use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct ParentCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
}

impl<'a> Command for ParentCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let parent_path = match self.dispatcher.path(self.current.path).parent() {
            Some(parent_path) => parent_path,
            None => return Ok(vec![]),
        };

        let paths: Paths = self.dispatcher.path_repository().list(&parent_path)?.into();
        let write_all = paths.to_write_all_action();

        let numbers = paths
            .into_iter()
            .enumerate()
            .filter(|(_, p)| &p.path == self.current.path)
            .map(|(line_number, _)| line_number + 1)
            .collect::<Vec<usize>>();

        let last_path_line_number = *numbers.get(0).unwrap_or(&0) as u64;

        Ok(vec![
            write_all,
            Action::SetCursor {
                line_number: last_path_line_number,
            },
            Action::SetPath { path: parent_path },
            Action::AddHistory {
                path: self.current.path.to_string(),
                line_number: self.current.line_number,
                back: false,
            },
        ])
    }
}
