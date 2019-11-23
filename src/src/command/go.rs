use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct GoCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub created: bool,
}

impl<'a> Command for GoCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let current_path = match &self.opts.path {
            Some(opt_path) => Path::new(opt_path),
            None => Path::new(self.current_path),
        };

        let paths: Paths = self.path_repository.list(current_path.to_str()?)?.into();

        let mut actions = vec![
            paths.to_write_all_action(),
            Action::TryToRestoreCursor {
                path: current_path.canonicalize()?.to_str()?.to_string(),
            },
            Action::AddHistory {
                path: self.current_path.to_string(),
                line_number: self.line_number,
            },
        ];
        if !self.created {
            actions.push(Action::Create);
        }

        Ok(actions)
    }
}
