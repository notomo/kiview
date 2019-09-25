use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct NewCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = Path::new(self.current_path);

        match &self.opts.path {
            Some(opt_path) => {
                let new_path = path.join(opt_path);
                match opt_path.ends_with("/") {
                    true => create_dir_all(new_path).and_then(|_| Ok(())),
                    false => File::create(new_path).and_then(|_| Ok(())),
                }
                .unwrap();

                let paths = self.path_repository.list(path.to_str().unwrap());

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
            None => vec![Action::ConfirmNew {}],
        }
    }
}
