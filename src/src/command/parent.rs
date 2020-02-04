use super::action::Paths;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::repository::PathRepository;

pub struct ParentCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
}

impl<'a> Command for ParentCommand<'a> {
    fn actions(&self) -> CommandResult {
        let parent_path = match self.repository.path(self.current.path).parent() {
            Some(parent_path) => parent_path,
            None => return Ok(vec![]),
        };

        let paths: Paths = self.repository.list(&parent_path)?.into();

        let mut actions = vec![
            paths.to_write_all_action(),
            Action::SetPath { path: parent_path },
            Action::AddHistory {
                path: self.current.path.to_string(),
                line_number: self.current.line_number,
                back: false,
            },
        ];

        let current_path = self.repository.path(self.current.path);
        if let Some(last_path_line_number) = paths.search(|p| current_path.equals(&p.path)) {
            actions.push(Action::SetCursor {
                line_number: last_path_line_number,
            });
        }

        Ok(actions)
    }
}
