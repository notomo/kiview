use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct ParentCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for ParentCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let current_path = Path::new(self.current_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or(self.current_path);

        let paths: Paths = self.path_repository.list(current_path)?.into();
        let write_all = paths.to_write_all_action();

        let numbers = &paths
            .into_iter()
            .enumerate()
            .filter(|(_, p)| &p.path == self.current_path)
            .map(|(line_number, _)| line_number + 1)
            .collect::<Vec<usize>>();

        let last_path_line_number = *numbers.get(0).unwrap_or(&0) as u64;

        Ok(vec![
            write_all,
            Action::SetCursor {
                line_number: last_path_line_number,
            },
            Action::SetPath {
                path: current_path.to_string(),
            },
            Action::AddHistory {
                path: self.current_path.to_string(),
                line_number: self.line_number,
            },
        ])
    }
}
