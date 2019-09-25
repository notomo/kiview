use std::fs::rename;
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct RenameCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub path_repository: &'a dyn PathRepository,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);

        match (self.opts.no_confirm, &self.opts.path, &self.current_target) {
            (true, Some(opt_path), Some(current_target)) => {
                let from = path.join(current_target);
                let to = path.join(opt_path);
                rename(from, to).and_then(|_| Ok(())).unwrap();

                let mut paths = self.path_repository.children(path.to_str().unwrap());
                paths.splice(0..0, vec!["..".to_string()]);

                let current_path = path.canonicalize().unwrap().to_str().unwrap().to_string();
                vec![Action::Update {
                    args: paths,
                    options: Action::options(
                        Some(current_path.clone()),
                        Some(current_path),
                        Some(self.line_number),
                        None,
                    ),
                }]
            }
            (false, _, Some(current_target)) => {
                let from = path.join(current_target).to_str().unwrap().to_string();
                vec![Action::ConfirmRename { arg: from }]
            }
            (_, _, _) => vec![Action::Unknown {}],
        }
    }
}
