use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
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
        let path = Path::new(self.current_path);

        Ok(match self.opts.no_confirm {
            true => {
                let files: Vec<_> = self
                    .targets
                    .iter()
                    .map(|target| path.join(target))
                    .filter(|path| {
                        path.metadata()
                            .and_then(|metadata| Ok(!metadata.is_dir()))
                            .unwrap_or(false)
                    })
                    .collect();
                for file in &files {
                    remove_file(file).unwrap();
                }

                let dirs: Vec<_> = self
                    .targets
                    .iter()
                    .map(|target| path.join(target))
                    .filter(|path| {
                        path.metadata()
                            .and_then(|metadata| Ok(metadata.is_dir()))
                            .unwrap_or(false)
                    })
                    .collect();
                for dir in &dirs {
                    remove_dir_all(dir)?;
                }

                let paths = self.path_repository.list(path.to_str()?)?;

                let current_path = path.canonicalize()?.to_str()?.to_string();
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
            false => vec![Action::ConfirmRemove],
        })
    }
}
