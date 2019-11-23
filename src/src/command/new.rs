use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct NewCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub opts: &'a CommandOptions,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match &self.opts.path {
            Some(opt_path) => {
                let new_path = Path::new(self.current_path).join(opt_path);
                match opt_path.ends_with("/") {
                    true => create_dir_all(new_path).and_then(|_| Ok(())),
                    false => File::create(new_path).and_then(|_| Ok(())),
                }?;

                let paths: Paths = self.path_repository.list(self.current_path)?.into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::RestoreCursor {
                        path: self.current_path.to_string(),
                        line_number: None,
                    },
                    Action::AddHistory {
                        path: self.current_path.to_string(),
                        line_number: self.line_number,
                    },
                ])
            }
            None => Ok(vec![Action::ConfirmNew]),
        }
    }
}
