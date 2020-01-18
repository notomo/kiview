use super::action::RenameItem;
use super::current::Target;
use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::repository::PathRepository;
use itertools::Itertools;

pub struct RenameCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let (target, path) = match (self.opts.no_confirm, &self.current.target, &self.opts.path) {
            (false, Some(target), _) => {
                return Ok(vec![Action::ConfirmRename {
                    relative_path: match self
                        .repository
                        .new_path(&target.path)
                        .to_relative(self.current.path)
                    {
                        Ok(relative_path) => relative_path,
                        Err(_) => target.path.clone(),
                    },
                    path: target.to_string(),
                }])
            }
            (true, Some(target), Some(path)) => (target, path),
            _ => {
                return Err(ErrorKind::Invalid {
                    message: String::from("no confirm rename required -path and -current-target"),
                }
                .into())
            }
        };

        let from = self.repository.new_path(&target.path).to_string()?;
        let target_group_path = match target.is_parent_node {
            false => self
                .repository
                .new_path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            true => self.current.path.to_string(),
        };
        let to = self.repository.new_path(&target_group_path).join(path)?;
        self.repository.rename(&from, &to)?;

        let paths: Paths = self.repository.children(&target_group_path)?.into();

        Ok(vec![paths.to_write_action(
            target.depth as usize,
            target.parent_id,
            self.current.target.as_ref().and_then(|t| t.last_sibling_id),
        )])
    }
}

pub struct MultipleRenameCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for MultipleRenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let targets = self
            .current
            .targets()
            .group_by(|target| target.depth)
            .into_iter()
            .fold(vec![], |mut acc: Vec<&Target>, (_, targets)| {
                let mut child_acc: Vec<_> = vec![];
                for target in targets {
                    let count = acc
                        .iter()
                        .filter(|x| self.repository.new_path(&target.path).contained(&x.path))
                        .count();
                    if count == 0 {
                        child_acc.push(target)
                    }
                }
                acc.extend(child_acc);
                acc
            });

        if self.current.rename_targets.len() == 0 && !self.current.renamer_opened {
            return Ok(vec![Action::OpenRenamer {
                path: self.current.path.to_string(),
                items: targets
                    .into_iter()
                    .map(|target| RenameItem {
                        id: target.id,
                        path: target.path.clone(),
                        relative_path: match self
                            .repository
                            .new_path(&target.path)
                            .to_relative(self.current.path)
                        {
                            Ok(relative_path) => relative_path,
                            Err(_) => target.path.clone(),
                        },
                        is_copy: false,
                    })
                    .collect(),
            }]);
        };
        if self.current.rename_targets.len() == 0 && self.current.renamer_opened {
            return Ok(vec![]);
        };

        let results: Vec<_> = self
            .current
            .rename_targets
            .iter()
            .map(|target| {
                match self.repository.rename_or_copy_with(
                    &target.from,
                    self.current.path,
                    &target.to,
                    target.is_copy,
                ) {
                    Ok(to) => Ok(RenameItem {
                        id: target.id,
                        path: to,
                        relative_path: target.to.clone(),
                        is_copy: false,
                    }),
                    Err(err) => Err(err),
                }
            })
            .collect();

        let actions = if results.iter().all(|res| res.is_ok()) {
            let paths: Paths = self.repository.list(self.current.path)?.into();
            vec![
                Action::CompleteRenamer {
                    items: results.into_iter().filter_map(|res| res.ok()).collect(),
                },
                paths.to_write_all_action(),
            ]
        } else {
            results
                .into_iter()
                .filter_map(|res| Action::show_error(res))
                .collect()
        };

        Ok(actions)
    }
}
