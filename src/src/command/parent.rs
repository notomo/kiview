use std::path::Path;

use crate::repository::PathRepository;

pub struct ParentCommand<'a> {
    pub current: &'a str,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> ParentCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current)
            .parent()
            .unwrap_or_else(|| Path::new(self.current));
        let paths = self.path_repository.children(path.to_str().unwrap());

        json!([{
            "name": "update",
            "args": paths,
            "options": {
                "cwd": path.canonicalize().unwrap(),
            },
        }])
    }
}
