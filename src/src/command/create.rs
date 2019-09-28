use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::repository::PathRepository;

pub struct CreateCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for CreateCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);
        let paths = self.path_repository.list(path.to_str().unwrap());

        let current_path = path.canonicalize().unwrap().to_str().unwrap().to_string();

        vec![
            Action::Write { paths: paths },
            Action::RestoreCursor {
                path: current_path.clone(),
                line_number: None,
            },
            Action::AddHistory {
                path: current_path,
                line_number: self.line_number,
            },
            Action::Create,
        ]
    }
}
