use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct RemoveCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for RemoveCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match self.opts.no_confirm {
            true => {
                let paths: Vec<_> = self
                    .targets
                    .iter()
                    .map(|target| Path::new(target))
                    .collect();

                let files: Vec<_> = paths
                    .iter()
                    .filter(|path| !path.to_path_buf().is_dir())
                    .collect();
                for file in &files {
                    remove_file(file)?;
                }

                let dirs: Vec<_> = paths
                    .iter()
                    .filter(|path| path.to_path_buf().is_dir())
                    .collect();
                for dir in &dirs {
                    remove_dir_all(dir)?;
                }

                let paths: Paths = self.path_repository.list(self.current_path)?.into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::TryToRestoreCursor {
                        path: self.current_path.to_string(),
                    },
                    Action::AddHistory {
                        path: self.current_path.to_string(),
                        line_number: self.line_number,
                    },
                ])
            }
            false => Ok(vec![Action::ConfirmRemove]),
        }
    }
}
