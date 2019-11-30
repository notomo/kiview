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
            (true, Some(opt_path), Some(target)) => {
                let from = self.dispatcher.path(&target.path).to_string()?;
                let target_group_path = match target.is_parent_node {
                    false => self
                        .dispatcher
                        .path(&target.path)
                        .parent()
                        .unwrap_or(target.path.clone()),
                    true => self.current.path.to_string(),
                };
                let to = self.dispatcher.path(&target_group_path).join(opt_path)?;
                self.dispatcher.path_repository().rename(&from, &to)?;

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
            (false, _, Some(target)) => {
                let from = target.to_string();
                Ok(vec![Action::ConfirmRename { path: from }])
            }
            _ => Err(crate::command::ErrorKind::Invalid {
                message: String::from("no confirm rename required -path and -current-target"),
            }
            .into()),
        }
    }
}
