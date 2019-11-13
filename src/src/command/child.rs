use std::path::Path;

use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::repository::PathRepository;

pub struct ChildCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub opts: &'a CommandOptions,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let path = Path::new(self.current_path);

        match self.current_target {
            Some(current_target)
                if path
                    .join(current_target)
                    .metadata()
                    .and_then(|metadata| Ok(metadata.is_dir()))
                    .unwrap_or(false) =>
            {
                let current_path = path.join(current_target);
                let paths = self.path_repository.list(current_path.to_str()?)?;

                Ok(vec![
                    Action::WriteAll { paths: paths },
                    Action::RestoreCursor {
                        path: current_path.canonicalize()?.to_str()?.to_string(),
                        line_number: None,
                    },
                    Action::AddHistory {
                        path: path.canonicalize()?.to_str()?.to_string(),
                        line_number: self.line_number,
                    },
                ])
            }
            _ => {
                let files: Vec<_> = self
                    .targets
                    .iter()
                    .map(|target| path.join(target))
                    .filter(|path| {
                        path.metadata()
                            .and_then(|metadata| Ok(!metadata.is_dir()))
                            .unwrap_or(false)
                    })
                    .map(|path| path.to_str().unwrap().to_string())
                    .collect();

                Ok(match self.opts.quit {
                    true => (vec![self.opts.layout.action(files), Action::Quit]),
                    false => vec![self.opts.layout.action(files)],
                })
            }
        }
    }
}
