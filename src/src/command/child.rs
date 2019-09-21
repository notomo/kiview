use std::path::Path;

use crate::repository::PathRepository;

pub struct ChildCommand<'a> {
    pub current_path: &'a str,
    pub current_target: Option<&'a str>,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> ChildCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current_path);

        match self.current_target {
            Some(current_target)
                if path
                    .join(current_target)
                    .metadata()
                    .and_then(|metadata| Ok(metadata.is_dir()))
                    .unwrap_or(false) =>
            {
                let path = path.join(current_target);
                let paths = self.path_repository.children(path.to_str().unwrap());
                json!([{
                  "name": "update",
                  "args": paths,
                  "options": {
                      "current_path": path.canonicalize().unwrap(),
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

                json!([{
                  "name": "open",
                  "args": files,
                  "options": {
                      "current_path": path.canonicalize().unwrap(),
                  },
                }])
            }
        }
    }
}
