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
        match self.opts.no_confirm {
            true => {
                let targets = self
                    .current
                    .targets()
                    .into_iter()
                    .map(|target| target.path)
                    .collect();

                self.dispatcher.path_repository().remove(targets)?;

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
            false => Ok(vec![Action::ConfirmRemove]),
        }
    }
}
