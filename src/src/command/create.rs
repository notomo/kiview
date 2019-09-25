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
        let mut paths: Vec<_> = self.path_repository.children(path.to_str().unwrap());
        paths.splice(0..0, vec!["..".to_string()]);

        let current_path = path.canonicalize().unwrap().to_str().unwrap().to_string();
        vec![Action::Create {
            args: paths,
            options: Action::options(
                Some(current_path.clone()),
                Some(current_path),
                Some(self.line_number),
                None,
            ),
        }]
    }
}
