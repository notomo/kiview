use std::fs::rename;
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct RenameCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match (self.opts.no_confirm, &self.opts.path, &self.current_target) {
            (true, Some(opt_path), Some(current_target)) => {
                let from = Path::new(current_target);
                let to = Path::new(self.current_path).join(opt_path);
                rename(from, to)?;

                let paths: Paths = self.path_repository.list(self.current_path)?.into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::RestoreCursor {
                        path: self.current_path.to_string(),
                        line_number: None,
                    },
                    Action::AddHistory {
                        path: self.current_path.to_string(),
                        line_number: self.line_number,
                    },
                ])
            }
            (false, _, Some(current_target)) => {
                let from = current_target.to_string();
                Ok(vec![Action::ConfirmRename { path: from }])
            }
            (_, _, _) => Ok(vec![Action::Unknown]),
        }
    }
}
