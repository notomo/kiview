use std::fs::{copy, rename};
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct PasteCommand<'a> {
    pub current: Current<'a>,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for PasteCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let from_paths: Vec<_> = self
            .current
            .registered_targets
            .iter()
            .map(|target| Path::new(target))
            .collect();

        // FIXME: when already exists
        for from in &from_paths {
            let to_name = from.file_name()?;
            let to = Path::new(self.current.path).join(to_name);
            match self.current.has_cut {
                true => rename(from, to).and_then(|_| Ok(())),
                false => copy(from, to).and_then(|_| Ok(())),
            }?;
        }

        let paths: Paths = self.path_repository.list(self.current.path)?.into();

        Ok(vec![
            paths.to_write_all_action(),
            Action::TryToRestoreCursor {
                path: self.current.path.to_string(),
            },
            Action::AddHistory {
                path: self.current.path.to_string(),
                line_number: self.current.line_number,
            },
            Action::ClearRegister,
        ])
    }
}
