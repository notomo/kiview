use super::action::Paths;
use super::action::RenameItem;
use super::command::CommandOption;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::{Error, ErrorKind};
use crate::repository::PathRepository;

pub struct RenameCommandOptions {
    no_confirm: bool,
    path: Option<String>,
}

impl From<Vec<CommandOption>> for RenameCommandOptions {
    fn from(opts: Vec<CommandOption>) -> Self {
        let mut no_confirm = false;
        let mut path = None;

        opts.into_iter().for_each(|opt| match opt {
            CommandOption::NoConfirm => no_confirm = true,
            CommandOption::Path { value } => path = Some(value),
            _ => (),
        });

        RenameCommandOptions {
            no_confirm: no_confirm,
            path: path,
        }
    }
}

pub struct RenameCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: RenameCommandOptions,
}

impl<'a> Command for RenameCommand<'a> {
    fn actions(&self) -> CommandResult {
        let (target, path) = match (self.opts.no_confirm, &self.current.target, &self.opts.path) {
            (false, Some(target), _) if !target.is_parent_node => {
                return Ok(vec![Action::ConfirmRename {
                    relative_path: self
                        .repository
                        .path(&target.path)
                        .to_relative(self.current.path)?,
                    path: target.to_string(),
                }])
            }
            (true, Some(target), Some(path)) => (target, path),
            (true, _, _) => {
                return Err(ErrorKind::invalid(
                    "no confirm rename required -path and -current-target",
                ))
            }
            _ => return Ok(vec![]),
        };

        let from = self.repository.path(&target.path).to_string()?;
        let target_group_path = match target.is_parent_node {
            false => self.repository.path(&target.path).parent()?,
            true => self.current.path.to_string(),
        };
        self.repository
            .rename_with(&from, &target_group_path, path, false)?;

        let paths: Paths = self.repository.children(&target_group_path)?.into();

        Ok(vec![paths.to_write_action(
            target.depth,
            target.parent_id,
            target.last_sibling_id,
        )])
    }
}

pub struct MultipleRenameCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
}

impl<'a> Command for MultipleRenameCommand<'a> {
    fn actions(&self) -> CommandResult {
        if self.current.rename_targets.len() == 0 && !self.current.renamer_opened {
            let items: Result<Vec<RenameItem>, Error> = self
                .current
                .dedup_targets(&self.repository, |target| !target.is_parent_node)
                .into_iter()
                .try_fold(vec![], |mut items, target| {
                    let relative_path = self
                        .repository
                        .path(&target.path)
                        .to_relative(self.current.path)?;
                    items.push(RenameItem {
                        id: target.id,
                        path: target.path.clone(),
                        relative_path: relative_path,
                        is_copy: false,
                    });
                    Ok(items)
                });
            return Ok(vec![Action::OpenRenamer {
                path: self.current.path.to_string(),
                items: items?,
            }]);
        } else if self.current.rename_targets.len() == 0 && self.current.renamer_opened {
            return Ok(vec![]);
        };

        let (items, errors) = self.current.rename_targets.iter().fold(
            (vec![], vec![]),
            |(mut items, mut errors), target| {
                match self.repository.rename_or_copy_with(
                    &target.from,
                    self.current.path,
                    &target.to,
                    target.is_copy,
                    false,
                ) {
                    Ok(to) => items.push(RenameItem {
                        id: target.id,
                        path: to,
                        relative_path: target.to.clone(),
                        is_copy: false,
                    }),
                    Err(err) => errors.push(Action::show_error(&target.to, err)),
                }
                (items, errors)
            },
        );

        let paths: Paths = self.repository.list(self.current.path)?.into();

        let mut actions = vec![
            Action::CompleteRenamer {
                items: items,
                has_error: errors.len() != 0,
            },
            paths.to_write_all_action(),
        ];
        actions.extend(errors);

        Ok(actions)
    }
}
