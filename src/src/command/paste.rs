use std::fs::{copy, rename};
use std::path::Path;

use crate::repository::PathRepository;

pub struct PasteCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub registered_targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
    pub has_cut: bool,
}

impl<'a> PasteCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let current_path = Path::new(self.current_path);

        let from_paths: Vec<_> = self
            .registered_targets
            .iter()
            .map(|target| Path::new(target))
            .collect();

        // FIXME: when already exists
        for from in &from_paths {
            let to_name = from.file_name().unwrap();
            let to = current_path.join(to_name);
            match self.has_cut {
                true => rename(from, to).and_then(|_| Ok(())),
                false => copy(from, to).and_then(|_| Ok(())),
            }
            .unwrap();
        }

        let mut paths = self
            .path_repository
            .children(current_path.to_str().unwrap());
        paths.splice(0..0, vec!["..".to_string()]);

        json!([
            {
                "name": "update",
                "args": paths,
                "options": {
                    "current_path": current_path.canonicalize().unwrap(),
                    "last_path": current_path.canonicalize().unwrap(),
                    "last_line_number": self.line_number,
                },
            },
            {
                "name": "clear_register",
                "args": [],
                "options": {},
            }
        ])
    }
}
