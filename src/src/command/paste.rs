use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct PasteCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
}

impl<'a> Command for PasteCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let from_paths: Vec<_> = self
            .current
            .registered_paths
            .iter()
            .map(|path| self.dispatcher.path(path))
            .collect();

        let repository = self.dispatcher.path_repository();
        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            Some(_) | None => self.current.path.to_string(),
        };

        // FIXME: when already exists
        for from_path in from_paths {
            let from = from_path.to_string()?;
            let to = self
                .dispatcher
                .path(&target_group_path)
                .join(&from_path.name()?)?;
            match self.current.has_cut {
                true => repository.rename(&from, &to),
                false => repository.copy(&from, &to),
            }?;
        }

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(&target_group_path)?
            .iter()
            .skip(1)
            .collect::<Vec<_>>()
            .into();

        Ok(vec![
            paths.to_write_action(
                self.current.depth as usize,
                self.current.parent_line_number as usize,
                self.current.last_sibling_line_number as usize,
            ),
            Action::ClearRegister,
        ])
    }
}
