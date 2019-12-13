use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct RemoveCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RemoveCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let targets = self
            .current
            .targets()
            .into_iter()
            .map(|target| target.path)
            .collect();

        if !self.opts.no_confirm {
            return Ok(vec![Action::ConfirmRemove { paths: targets }]);
        }

        self.dispatcher.path_repository().remove(targets)?;

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(self.current.path)?
            .into();

        Ok(vec![paths.to_write_all_action()])
    }
}
