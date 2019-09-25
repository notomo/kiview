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
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for RemoveCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);

        match self.opts.no_confirm {
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
                    remove_dir_all(dir).unwrap();
                }

                let mut paths = self.path_repository.children(path.to_str().unwrap());
                paths.splice(0..0, vec!["..".to_string()]);

                let current_path = path.canonicalize().unwrap().to_str().unwrap().to_string();
                vec![Action::Update {
                    args: paths,
                    options: Action::options(
                        Some(current_path.clone()),
                        Some(current_path),
                        Some(self.line_number),
                        None,
                    ),
                }]
            }
            false => vec![Action::ConfirmRemove {}],
        }
    }
}
