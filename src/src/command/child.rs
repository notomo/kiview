use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;

use super::command::{Layout, SplitModName, SplitName};

use itertools::Itertools;

pub struct ChildCommand<'a> {
    pub current: Current<'a>,
    pub opts: &'a CommandOptions,
    pub dispatcher: Dispatcher,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let mut actions: Vec<_> = self
            .current
            .targets()
            .iter()
            .group_by(|target| self.dispatcher.path(&target.path).is_group_node())
            .into_iter()
            .flat_map(|(is_group_node, targets)| {
                let paths = targets
                    .into_iter()
                    .map(|target| target.to_string())
                    .collect();
                match (is_group_node, self.opts.layout) {
                    (false, _) => vec![self.opts.layout.leaf_node_action(paths)],
                    (true, Layout::Open) => {
                        let results: Vec<_> = paths
                            .iter()
                            .map(|p| (p, self.dispatcher.path_repository().list(&p)))
                            .collect();

                        let mut error_actions: Vec<_> = results
                            .iter()
                            .filter(|(_, res)| res.is_err())
                            .map(|(p, res)| Action::ShowError {
                                path: p.to_string(),
                                message: res.as_ref().err().unwrap().inner.to_string(),
                            })
                            .collect();

                        let mut actions: Vec<_> = results
                            .iter()
                            .filter(|(_, res)| res.is_ok())
                            .flat_map(|(p, res)| {
                                vec![
                                    Paths::from(res.as_ref().unwrap()).to_write_all_action(),
                                    Action::TryToRestoreCursor {
                                        path: p.to_string(),
                                    },
                                ]
                            })
                            .collect();

                        error_actions.append(&mut actions);
                        error_actions.push(Action::AddHistory {
                            path: self.current.path.to_string(),
                            line_number: self.current.line_number,
                            back: false,
                        });
                        error_actions
                    }
                    _ => {
                        let results: Vec<_> = paths
                            .iter()
                            .map(|p| (p, self.dispatcher.path_repository().list(&p)))
                            .collect();

                        let items: Vec<_> = results
                            .iter()
                            .filter(|(_, res)| res.is_ok())
                            .map(|(p, res)| {
                                Paths::from(res.as_ref().unwrap()).to_fork_buffer_item(p)
                            })
                            .collect();

                        let mut actions: Vec<_> = results
                            .iter()
                            .filter(|(_, res)| res.is_err())
                            .map(|(p, res)| Action::ShowError {
                                path: p.to_string(),
                                message: res.as_ref().err().unwrap().inner.to_string(),
                            })
                            .collect();

                        actions.push(Action::ForkBuffer {
                            items: items,
                            mod_name: SplitModName::No,
                            split_name: SplitName::from(self.opts.layout),
                        });
                        actions
                    }
                }
            })
            .collect();

        if self.opts.quit {
            actions.push(Action::Quit);
        }

        Ok(actions)
    }
}
