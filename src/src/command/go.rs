use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::PathRepository;
use std::path::Path;

pub struct GoCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub created: bool,
}

impl<'a> Command for GoCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let current_path = Path::new(match &self.opts.path {
            Some(opt_path) => opt_path.as_str(),
            None => self.current_path,
        })
        .canonicalize()?
        .to_str()?
        .to_string();

        let paths: Paths = self.path_repository.list(&current_path)?.into();

        let mut actions = vec![
            paths.to_write_all_action(),
            Action::TryToRestoreCursor { path: current_path },
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
