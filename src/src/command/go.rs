use std::path::Path;

use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct GoCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> GoCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);

        let current_path = match &self.opts.path {
            Some(opt_path) => Path::new(opt_path),
            None => path,
        };

        let mut paths = self
            .path_repository
            .children(current_path.to_str().unwrap());
        paths.splice(0..0, vec!["..".to_string()]);

        json!([{
            "name": "update",
            "args": paths,
            "options": {
                "current_path": current_path.canonicalize().unwrap(),
                "last_path": path.canonicalize().unwrap(),
                "last_line_number": self.line_number,
            },
        }])
    }
}
