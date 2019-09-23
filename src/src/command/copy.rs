use std::path::Path;

use crate::repository::PathRepository;

pub struct CopyCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> CopyCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);

        let paths: Vec<_> = self
            .targets
            .iter()
            .map(|target| path.join(target).canonicalize().unwrap())
            .collect();

        json!([{
            "name": "copy",
            "args": paths,
            "options": {},
        }])
    }
}
