use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct NewCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        if self.opts.path.is_none() {
            return Ok(vec![Action::ConfirmNew]);
        }

        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            Some(_) | None => self.current.path.to_string(),
        };
        let new_path = self
            .dispatcher
            .path(&target_group_path)
            .join(self.opts.path.as_ref().unwrap())?;

        self.dispatcher.path_repository().create(&new_path)?;

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
