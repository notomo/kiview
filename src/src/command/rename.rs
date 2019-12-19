use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::repository::Dispatcher;

pub struct RenameCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let (target, path) = match (self.opts.no_confirm, &self.current.target, &self.opts.path) {
            (false, Some(target), _) => {
                return Ok(vec![Action::ConfirmRename {
                    path: target.to_string(),
                }])
            }
            (true, Some(target), Some(path)) => (target, path),
            _ => {
                return Err(ErrorKind::Invalid {
                    message: String::from("no confirm rename required -path and -current-target"),
                }
                .into())
            }
        };

        let from = self.dispatcher.path(&target.path).to_string()?;
        let target_group_path = match target.is_parent_node {
            false => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            true => self.current.path.to_string(),
        };
        let to = self.dispatcher.path(&target_group_path).join(path)?;
        self.dispatcher.path_repository().rename(&from, &to)?;

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(&target_group_path)?
            .iter()
            .skip(1)
            .collect::<Vec<_>>()
            .into();

        Ok(vec![paths.to_write_action(
            self.current.depth as usize,
            self.current.parent_line_number as usize,
            self.current.last_sibling_line_number as usize,
        )])
    }
}
