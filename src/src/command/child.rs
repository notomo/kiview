use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::PathRepository;

use super::command::{Layout, SplitModName, SplitName};

use itertools::Itertools;

pub struct ChildCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let mut actions: Vec<_> = self
            .current
            .targets()
            .group_by(|target| self.repository.path(&target.path).is_group_node())
            .into_iter()
            .flat_map(|(is_group_node, targets)| {
                let paths = targets
                    .into_iter()
                    .map(|target| target.to_string())
                    .collect();

                match (is_group_node, self.opts.layout) {
                    (false, _) => vec![self.opts.layout.leaf_node_action(paths)],
                    (true, Layout::Open) => paths
                        .iter()
                        .flat_map(|path| match self.repository.list(&path) {
                            Ok(paths) => vec![
                                Paths::from(paths).to_write_all_action(),
                                Action::TryToRestoreCursor {
                                    path: path.to_string(),
                                },
                            ],
                            Err(err) => vec![Action::ShowError {
                                path: path.to_string(),
                                message: err.inner.to_string(),
                            }],
                        })
                        .chain(vec![Action::AddHistory {
                            path: self.current.path.to_string(),
                            line_number: self.current.line_number,
                            back: false,
                        }])
                        .collect(),
                    (true, _) => {
                        let (items, errors) =
                            paths
                                .iter()
                                .fold((vec![], vec![]), |(mut items, mut errors), path| {
                                    match self.repository.list(&path) {
                                        Ok(paths) => {
                                            let item = Paths::from(paths).to_fork_buffer_item(path);
                                            items.push(item);
                                        }
                                        Err(err) => errors.push(Action::ShowError {
                                            path: path.to_string(),
                                            message: err.inner.to_string(),
                                        }),
                                    };
                                    (items, errors)
                                });

                        let mut actions = vec![Action::ForkBuffer {
                            items: items,
                            mod_name: SplitModName::No,
                            split_name: SplitName::from(self.opts.layout),
                        }];
                        actions.extend(errors);
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
