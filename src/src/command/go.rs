use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct GoCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for GoCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);

        let current_path = match &self.opts.path {
            Some(opt_path) => Path::new(opt_path),
            None => path,
        };

        let paths = self.path_repository.list(current_path.to_str().unwrap());

        vec![
            Action::Write { paths: paths },
            Action::RestoreCursor {
                path: current_path
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                line_number: None,
            },
            Action::AddHistory {
                path: path.canonicalize().unwrap().to_str().unwrap().to_string(),
                line_number: self.line_number,
            },
        ]
    }
}
