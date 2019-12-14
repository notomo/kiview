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
        if !self.opts.no_confirm && self.current.target.is_some() {
            let from = self.current.target.as_ref().unwrap().to_string();
            return Ok(vec![Action::ConfirmRename { path: from }]);
        }

        if !self.opts.no_confirm && (self.opts.path.is_none() && self.current.target.is_none()) {
            return Err(ErrorKind::Invalid {
                message: String::from("no confirm rename required -path and -current-target"),
            }
            .into());
        }

        let target = self.current.target.as_ref().unwrap();
        let opt_path = self.opts.path.as_ref().unwrap();

        let from = self.dispatcher.path(&target.path).to_string()?;
        let target_group_path = match target.is_parent_node {
            false => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            true => self.current.path.to_string(),
        };
        let to = self.dispatcher.path(&target_group_path).join(&opt_path)?;
        self.dispatcher.path_repository().rename(&from, &to)?;

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(self.current.path)?
            .into();

        Ok(vec![paths.to_write_all_action()])
    }
}
