use super::action::Paths;
use super::command::CommandResult;
use super::command::{CommandOption, Split, SplitModName, SplitName};
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::repository::PathRepository;

use itertools::Itertools;

pub struct ChildCommandOptions {
    split: Split,
    quit: bool,
}

impl From<Vec<CommandOption>> for ChildCommandOptions {
    fn from(opts: Vec<CommandOption>) -> Self {
        let mut split = Split {
            name: SplitName::Open,
            mod_name: SplitModName::No,
        };
        let mut quit = false;
        opts.into_iter().for_each(|opt| match opt {
            CommandOption::Split { value } => split = value,
            CommandOption::Quit => quit = true,
            _ => (),
        });
        ChildCommandOptions {
            split: split,
            quit: quit,
        }
    }
}

pub struct ChildCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: ChildCommandOptions,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> CommandResult {
        let actions: Vec<_> = self
            .current
            .targets()
            .group_by(|target| self.repository.path(&target.path).is_group_node())
            .into_iter()
            .flat_map(|(is_group_node, targets)| {
                let paths = targets
                    .into_iter()
                    .map(|target| target.to_string())
                    .collect();

                match (is_group_node, self.opts.split.name) {
                    (false, _) => {
                        let mut actions = self.opts.split.leaf_node_action(paths);
                        if self.opts.quit {
                            actions.push(Action::Quit);
                        };
                        actions
                    }
                    (true, SplitName::Open) => paths
                        .iter()
                        .flat_map(|path| match self.repository.list(&path) {
                            Ok(paths) => vec![
                                Paths::from(paths).to_write_all_action(),
                                Action::TryToRestoreCursor {
                                    path: path.to_string(),
                                },
                            ],
                            Err(err) => vec![Action::show_error(&path, err)],
                        })
                        .chain(vec![Action::AddHistory {
                            path: self.current.path.to_string(),
                            line_number: self.current.line_number,
                            back: false,
                        }])
                        .collect(),
                    (true, _) => {
                        let (mut actions, errors) =
                            paths
                                .iter()
                                .fold((vec![], vec![]), |(mut items, mut errors), path| {
                                    match self.repository.list(&path) {
                                        Ok(paths) => {
                                            let item = Paths::from(paths)
                                                .to_fork_buffer(path, self.opts.split);
                                            items.push(item);
                                        }
                                        Err(err) => errors.push(Action::show_error(&path, err)),
                                    };
                                    (items, errors)
                                });

                        actions.extend(errors);
                        actions
                    }
                }
            })
            .chain(vec![Action::UnselectAll])
            .collect();

        Ok(actions)
    }
}
