use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::repository::PathRepository;

pub struct CutCommand<'a> {
    pub current_path: &'a str,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);

        let paths: Vec<_> = self
            .targets
            .iter()
            .map(|target| {
                path.join(target)
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect();

        vec![Action::Cut { args: paths }]
    }
}
