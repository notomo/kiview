use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct ToggleTreeCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub opts: &'a CommandOptions,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub next_sibling_line_number: u64,
    pub depth: u64,
}

impl<'a> Command for ToggleTreeCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let path = Path::new(self.current_path);

        if self.line_number == 1 {
            return Ok(vec![]);
        }

        if self.next_sibling_line_number > self.line_number + 1 {
            return Ok(vec![Action::Write {
                paths: vec![],
                start: self.line_number as usize,
                end: (self.next_sibling_line_number - 1) as usize,
            }]);
        }

        match self.current_target {
            Some(current_target)
                if path
                    .join(current_target)
                    .metadata()
                    .and_then(|metadata| Ok(metadata.is_dir()))
                    .unwrap_or(false) =>
            {
                let indent = std::iter::repeat(" ")
                    .take(self.depth as usize)
                    .collect::<String>();

                let target_path = path.join(current_target);
                let child_paths = self
                    .path_repository
                    .list(target_path.to_str()?)?
                    .iter()
                    .skip(1)
                    .map(|p| format!("{}  {}", indent, p))
                    .collect::<Vec<String>>();

                Ok(vec![Action::Write {
                    paths: child_paths,
                    start: self.line_number as usize,
                    end: self.line_number as usize,
                }])
            }
            _ => Ok(vec![]),
        }
    }
}
