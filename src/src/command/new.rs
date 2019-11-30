use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct NewCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match &self.opts.path {
            Some(opt_path) => {
                let target_group_path = match &self.current.target {
                    Some(target) if !target.is_parent_node => self
                        .dispatcher
                        .path(&target.path)
                        .parent()
                        .unwrap_or(target.path.clone()),
                    Some(_) | None => self.current.path.to_string(),
                };
                let new_path = self.dispatcher.path(&target_group_path).join(opt_path)?;

                self.dispatcher.path_repository().create(&new_path)?;

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
            None => Ok(vec![Action::ConfirmNew]),
        }
    }
}
