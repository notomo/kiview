use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::repository::PathRepository;

pub struct ParentCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for ParentCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);
        let last_target: String = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| format!("{}/", name))
            .unwrap_or_else(|| "".to_string());

        let current_path = path
            .parent()
            .unwrap_or_else(|| Path::new(self.current_path));
        let paths = self.path_repository.list(current_path.to_str().unwrap());

        let numbers = &paths
            .iter()
            .enumerate()
            .filter(|(_, path)| *path == &last_target)
            .map(|(line_number, _)| line_number + 1)
            .collect::<Vec<usize>>();

        let last_path_line_number = *numbers.get(0).unwrap_or(&0) as u64;

        vec![
            Action::Write { paths: paths },
            Action::RestoreCursor {
                path: current_path
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                line_number: Some(last_path_line_number),
            },
            Action::AddHistory {
                path: path.canonicalize().unwrap().to_str().unwrap().to_string(),
                line_number: self.line_number,
            },
        ]
    }
}
