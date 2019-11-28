use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct ChildCommand<'a> {
    pub current: Current<'a>,
    pub opts: &'a CommandOptions,
    pub dispatcher: Dispatcher,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match &self.current.target {
            Some(target) if self.dispatcher.path(&target.path).is_group_node() => {
                let paths: Paths = self.dispatcher.path_repository().list(&target.path)?.into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::TryToRestoreCursor {
                        path: target.to_string(),
                    },
                    Action::AddHistory {
                        path: target.to_string(),
                        line_number: self.current.line_number,
                    },
                ])
            }
            _ => {
                let leaves: Vec<_> = self
                    .current
                    .targets()
                    .iter()
                    .filter(|target| !self.dispatcher.path(&target.path).is_group_node())
                    .map(|target| target.to_string())
                    .collect();

                Ok(match self.opts.quit {
                    true => vec![self.opts.layout.action(leaves), Action::Quit],
                    false => vec![self.opts.layout.action(leaves)],
                })
            }
        }
    }
}
