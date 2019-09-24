use std::path::Path;

use crate::command::Command;
use crate::repository::PathRepository;

pub struct CutCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);

        let paths: Vec<_> = self
            .targets
            .iter()
            .map(|target| path.join(target).canonicalize().unwrap())
            .collect();

        json!([{
            "name": "cut",
            "args": paths,
            "options": {},
        }])
    }
}
