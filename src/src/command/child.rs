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
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> Command for ChildCommand<'a> {
    fn actions(&self) -> Vec<Action> {
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
                let paths = self.path_repository.list(current_path.to_str().unwrap());

                vec![Action::Update {
                    args: paths,
                    options: Action::options(
                        Some(
                            current_path
                                .canonicalize()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
                        Some(path.canonicalize().unwrap().to_str().unwrap().to_string()),
                        Some(self.line_number),
                        None,
                    ),
                }]
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

                match self.opts.quit {
                    true => vec![self.opts.layout.action(files), Action::Quit {}],
                    false => vec![self.opts.layout.action(files)],
                }
            }
        }
    }
}
