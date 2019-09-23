use std::path::Path;

use crate::repository::PathRepository;

pub struct ParentCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> ParentCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);
        let last_target: String = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| format!("{}/", name))
            .unwrap_or_else(|| "".to_string());

        let current_path = path
            .parent()
            .unwrap_or_else(|| Path::new(self.current_path));
        let mut paths = self
            .path_repository
            .children(current_path.to_str().unwrap());
        paths.splice(0..0, vec!["..".to_string()]);

        let numbers = &paths
            .iter()
            .enumerate()
            .filter(|(_, path)| *path == &last_target)
            .map(|(line_number, _)| line_number + 1)
            .collect::<Vec<usize>>();

        let last_path_line_number = numbers.get(0).unwrap_or(&0);

        json!([{
            "name": "update",
            "args": paths,
            "options": {
                "current_path": current_path.canonicalize().unwrap(),
                "last_path": path.canonicalize().unwrap(),
                "last_line_number": self.line_number,
                "last_path_line_number": last_path_line_number,
            },
        }])
    }
}
