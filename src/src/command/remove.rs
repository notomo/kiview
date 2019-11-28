use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct RemoveCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RemoveCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match self.opts.no_confirm {
            true => {
                let targets = self.current.targets();
                let paths: Vec<_> = targets
                    .iter()
                    .map(|target| Path::new(&target.path))
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

                let paths: Paths = self
                    .dispatcher
                    .path_repository()
                    .list(self.current.path)?
                    .into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::TryToRestoreCursor {
                        path: self.current.path.to_string(),
                    },
                    Action::AddHistory {
                        path: self.current.path.to_string(),
                        line_number: self.current.line_number,
                    },
                ])
            }
            false => Ok(vec![Action::ConfirmRemove]),
        }
    }
}
