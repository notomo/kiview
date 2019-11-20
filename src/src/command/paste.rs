use std::fs::{copy, rename};
use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct PasteCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub registered_targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub has_cut: bool,
}

impl<'a> Command for PasteCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let current_path = Path::new(self.current_path);

        let from_paths: Vec<_> = self
            .registered_targets
            .iter()
            .map(|target| Path::new(target))
            .collect();

        // FIXME: when already exists
        for from in &from_paths {
            let to_name = from.file_name()?;
            let to = current_path.join(to_name);
            match self.has_cut {
                true => rename(from, to).and_then(|_| Ok(())),
                false => copy(from, to).and_then(|_| Ok(())),
            }?;
        }

        let paths: Paths = self.path_repository.list(current_path.to_str()?)?.into();

        let path = current_path.canonicalize()?.to_str()?.to_string();

        Ok(vec![
            paths.to_write_all_action(),
            Action::RestoreCursor {
                path: path.clone(),
                line_number: None,
            },
            Action::AddHistory {
                path: path,
                line_number: self.line_number,
            },
            Action::ClearRegister,
        ])
    }
}
