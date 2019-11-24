use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::{Dispatcher, PathRepository};

pub struct ChildCommand<'a> {
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub opts: &'a CommandOptions,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub dispatcher: Dispatcher,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        match self.current_target {
            Some(target) if self.dispatcher.path(target).is_group_node() => {
                let paths: Paths = self.path_repository.list(target)?.into();

                Ok(vec![
                    paths.to_write_all_action(),
                    Action::TryToRestoreCursor {
                        path: target.to_string(),
                    },
                    Action::AddHistory {
                        path: target.to_string(),
                        line_number: self.line_number,
                    },
                ])
            }
            _ => {
                let leaves: Vec<_> = self
                    .targets
                    .iter()
                    .filter(|target| !self.dispatcher.path(target).is_group_node())
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
