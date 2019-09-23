use std::path::Path;

use crate::repository::PathRepository;

pub struct CreateCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> CreateCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);
        let mut paths: Vec<_> = self.path_repository.children(path.to_str().unwrap());
        paths.splice(0..0, vec!["..".to_string()]);

        json!([{
            "name": "create",
            "args": paths,
            "options": {
                "current_path": path.canonicalize().unwrap(),
                "last_path": path.canonicalize().unwrap(),
                "last_line_number": self.line_number,
            },
        }])
    }
}
