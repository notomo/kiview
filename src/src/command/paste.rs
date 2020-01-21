use super::action::ChooseItem;
use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::itertools::Itertools;
use crate::repository::PathRepository;

pub struct PasteCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for PasteCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .repository
                .new_path(&target.path)
                .parent()
                .unwrap_or_else(|| self.current.path.to_string()),
            Some(_) | None => self.current.path.to_string(),
        };

        let (items, errors): (Vec<ChooseItem>, Vec<Action>) = self
            .current
            .registered_targets
            .iter()
            .unique_by(|target| match &target.from {
                Some(from) => from,
                None => &target.path,
            })
            .try_fold((vec![], vec![]), |(mut items, mut errors), target| {
                let from = self.repository.new_path(match &target.from {
                    Some(from) => from,
                    None => &target.path,
                });
                let from_name = from.name();
                let from = from.to_string()?;

                let new_name = match (&target.new_name, &from_name) {
                    (Some(name), _) => name,
                    (None, Some(name)) => name,
                    _ => return Err(ErrorKind::invalid("new name not found")),
                };
                let to = self
                    .repository
                    .new_path(&target_group_path)
                    .join(&new_name)?;

                match (self.opts.no_confirm, self.repository.new_path(&to).exists()) {
                    (false, true) => items.push(ChooseItem {
                        relative_path: self
                            .repository
                            .new_path(&to)
                            .to_relative(self.current.path)?,
                        path: to.clone(),
                        from: from,
                    }),
                    _ => {
                        if let Err(err) =
                            self.repository
                                .rename_or_copy(&from, &to, !self.current.has_cut)
                        {
                            errors.push(Action::ShowError {
                                path: to.clone(),
                                message: err.inner.to_string(),
                            })
                        }
                    }
                };
                Ok((items, errors))
            })?;

        let paths: Paths = self.repository.children(&target_group_path)?.into();

        let actions: Vec<_> = vec![
            paths.to_write_action(
                match &self.current.target {
                    Some(target) => target.depth,
                    None => 0,
                },
                self.current.target.as_ref().and_then(|t| t.parent_id),
                self.current.target.as_ref().and_then(|t| t.last_sibling_id),
            ),
            Action::ClearRegister,
        ]
        .into_iter()
        .chain(errors)
        .chain(vec![Action::Choose {
            path: self.current.path.to_string(),
            items: items,
            has_cut: self.current.has_cut,
        }])
        .collect();
        Ok(actions)
    }
}
