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
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let path = Path::new(self.current_path);

        Ok(match &self.opts.path {
            Some(opt_path) => {
                let new_path = path.join(opt_path);
                match opt_path.ends_with("/") {
                    true => create_dir_all(new_path).and_then(|_| Ok(())),
                    false => File::create(new_path).and_then(|_| Ok(())),
                }?;

                let paths = self.path_repository.list(path.to_str()?)?;

                let current_path = path.canonicalize()?.to_str()?.to_string();

                vec![
                    Action::WriteAll { paths: paths },
                    Action::RestoreCursor {
                        path: current_path.clone(),
                        line_number: None,
                    },
                    Action::AddHistory {
                        path: current_path,
                        line_number: self.line_number,
                    },
                ]
            }
            None => vec![Action::ConfirmNew],
        })
    }
}
