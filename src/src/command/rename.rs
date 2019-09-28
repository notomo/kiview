use std::fs::rename;
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct RenameCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);

        match (self.opts.no_confirm, &self.opts.path, &self.current_target) {
            (true, Some(opt_path), Some(current_target)) => {
                let from = path.join(current_target);
                let to = path.join(opt_path);
                rename(from, to).and_then(|_| Ok(())).unwrap();

                let paths = self.path_repository.list(path.to_str().unwrap());

                let current_path = path.canonicalize().unwrap().to_str().unwrap().to_string();
                vec![
                    Action::Write { paths: paths },
                    Action::RestoreCursor {
                        path: current_path.clone(),
                        line_number: None,
                    },
                    Action::AddHistory {
                        path: current_path,
                        line_number: self.line_number,
                    },
                ]
            }
            (false, _, Some(current_target)) => {
                let from = path.join(current_target).to_str().unwrap().to_string();
                vec![Action::ConfirmRename { path: from }]
            }
            (_, _, _) => vec![Action::Unknown],
        }
    }
}
