use std::fs::rename;
use std::path::Path;

use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct RenameCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub path_repository: &'a dyn PathRepository,
    pub opts: &'a CommandOptions,
}

impl<'a> RenameCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);

        match (self.opts.no_confirm, &self.opts.path, &self.current_target) {
            (true, Some(opt_path), Some(current_target)) => {
                let from = path.join(current_target);
                let to = path.join(opt_path);
                rename(from, to).and_then(|_| Ok(())).unwrap();

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
            (false, _, Some(current_target)) => {
                let from = path.join(current_target);
                json!([{
                    "name": "confirm_rename",
                    "args": [from],
                    "options": {},
                }])
            }
            (_, _, _) => json!([]),
        }
    }
}
