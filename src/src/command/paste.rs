use super::action::ChooseItem;
use super::action::Paths;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::ErrorKind;
use crate::itertools::Itertools;
use crate::repository::PathRepository;

pub struct PasteCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
}

impl<'a> Command for PasteCommand<'a> {
    fn actions(&self) -> CommandResult {
        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .repository
                .path(&target.path)
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
                let from = self.repository.path(match &target.from {
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
                let to = self.repository.path(&target_group_path).join(&new_name)?;

                match (target.force, self.repository.path(&to).exists()) {
                    (false, true) => items.push(ChooseItem {
                        relative_path: self.repository.path(&to).to_relative(self.current.path)?,
                        path: to.clone(),
                        from: from,
                    }),
                    _ => {
                        if let Err(err) = self.repository.rename_or_copy(
                            &from,
                            &to,
                            !self.current.has_cut,
                            target.force,
                        ) {
                            errors.push(Action::show_error(&to, err))
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
