use std::fs::rename;
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct RenameCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match (self.opts.no_confirm, &self.opts.path, &self.current.target) {
            (true, Some(opt_path), Some(current_target)) => {
                let from = Path::new(&current_target.path);
                let to = Path::new(self.current.path).join(opt_path);
                rename(from, to)?;

                let paths: Paths = self
                    .dispatcher
                    .path_repository()
                    .list(self.current.path)?
                    .into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::TryToRestoreCursor {
                        path: self.current.path.to_string(),
                    },
                    Action::AddHistory {
                        path: self.current.path.to_string(),
                        line_number: self.current.line_number,
                    },
                ])
            }
            (false, _, Some(current_target)) => {
                let from = current_target.to_string();
                Ok(vec![Action::ConfirmRename { path: from }])
            }
            _ => Err(crate::command::ErrorKind::Invalid {
                message: String::from("no confirm rename required -path and -current-target"),
            }
            .into()),
        }
    }
}
