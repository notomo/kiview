use super::action::RenameItem;
use super::current::Target;
use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::repository::Dispatcher;
use itertools::Itertools;

pub struct RenameCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let (target, path) = match (self.opts.no_confirm, &self.current.target, &self.opts.path) {
            (false, Some(target), _) => {
                return Ok(vec![Action::ConfirmRename {
                    relative_path: match self
                        .dispatcher
                        .path(&target.path)
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

        let from = self.dispatcher.path(&target.path).to_string()?;
        let target_group_path = match target.is_parent_node {
            false => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            true => self.current.path.to_string(),
        };
        let to = self.dispatcher.path(&target_group_path).join(path)?;
        self.dispatcher.path_repository().rename(&from, &to)?;

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(&target_group_path)?
            .iter()
            .skip(1)
            .collect::<Vec<_>>()
            .into();

        Ok(vec![paths.to_write_action(
            target.depth as usize,
            target.parent_id,
            self.current.target.as_ref().and_then(|t| t.last_sibling_id),
        )])
    }
}

pub struct MultipleRenameCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for MultipleRenameCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let targets = self
            .current
            .targets()
            .into_iter()
            .group_by(|target| target.depth)
            .into_iter()
            .fold(vec![], |mut acc: Vec<Target>, (_, targets)| {
                let mut child_acc: Vec<_> = vec![];
                for target in targets {
                    let count = acc
                        .iter()
                        .filter(|x| self.dispatcher.path(&target.path).contained(&x.path))
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
                            .dispatcher
                            .path(&target.path)
                            .to_relative(self.current.path)
                        {
                            Ok(relative_path) => relative_path,
                            Err(_) => target.path.clone(),
                        },
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
                let to = match self.dispatcher.path(self.current.path).join(&target.to) {
                    Ok(to) => to,
                    Err(err) => return Err(Error::from(err)),
                };
                match self.dispatcher.path_repository().rename(&target.from, &to) {
                    Ok(()) => Ok(RenameItem {
                        id: target.id,
                        path: to,
                        relative_path: target.to.clone(),
                    }),
                    Err(err) => Err(Error::from(err)),
                }
            })
            .collect();

        let mut actions: Vec<_> = results
            .iter()
            .filter(|res| res.is_err())
            .map(|res| Action::ShowError {
                path: String::from(""),
                message: res.as_ref().err().unwrap().inner.to_string(),
            })
            .collect();

        if actions.len() == 0 {
            actions.push(Action::CompleteRenamer {
                items: results
                    .into_iter()
                    .filter(|res| res.is_ok())
                    .map(|res| res.as_ref().unwrap().clone())
                    .collect(),
            })
        }

        Ok(actions)
    }
}
