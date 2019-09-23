use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct RemoveCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> RemoveCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
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

                json!([{
                    "name": "update",
                    "args": paths,
                    "options": {
                        "current_path": path.canonicalize().unwrap(),
                        "last_path": path.canonicalize().unwrap(),
                        "last_line_number": self.line_number,
                    },
                }])
            }
            false => json!([{
                "name": "confirm_remove",
                "args": [],
                "options": {
                    "current_path": path.canonicalize().unwrap(),
                    "last_path": path.canonicalize().unwrap(),
                    "last_line_number": self.line_number,
                },
            }]),
        }
    }
}
