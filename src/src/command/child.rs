use std::path::Path;

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
    fn actions(&self) -> serde_json::Value {
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
                let mut paths = self
                    .path_repository
                    .children(current_path.to_str().unwrap());
                paths.splice(0..0, vec!["..".to_string()]);

                json!([{
                  "name": "update",
                  "args": paths,
                  "options": {
                      "current_path": current_path.canonicalize().unwrap(),
                      "last_path": path.canonicalize().unwrap(),
                      "last_line_number": self.line_number,
                  }
                }])
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
                    .collect();

                let action_name = match self.opts.layout {
                    Some(layout) => layout.action(),
                    None => "open".to_string(),
                };

                match self.opts.quit {
                    true => json!([
                        {
                            "name": action_name,
                            "args": files,
                            "options": {},
                        },
                        {
                            "name": "quit",
                            "args": [],
                            "options": {},
                        }
                    ]),
                    false => json!([{
                      "name": action_name,
                      "args": files,
                      "options": {},
                    }]),
                }
            }
        }
    }
}
