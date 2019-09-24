use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;

use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct NewCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);

        match &self.opts.path {
            Some(opt_path) => {
                let new_path = path.join(opt_path);
                match opt_path.ends_with("/") {
                    true => create_dir_all(new_path).and_then(|_| Ok(())),
                    false => File::create(new_path).and_then(|_| Ok(())),
                }
                .unwrap();

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
            None => json!([{
                "name": "confirm_new",
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
