use std::path::Path;

use crate::repository::PathRepository;

pub struct ChildCommand<'a> {
    pub current: &'a str,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository,
}

impl<'a> ChildCommand<'a> {
    pub fn actions(&self) -> serde_json::Value {
        let path = Path::new(self.current);
        let dirs: Vec<_> = self
            .targets
            .iter()
            .map(|target| Path::new(self.current).join(target))
            .filter(|path| {
                path.metadata()
                    .and_then(|metadata| Ok(metadata.is_dir()))
                    .unwrap_or(false)
            })
            .collect();

        match &dirs[..] {
            [] => {
                let files: Vec<_> = self
                    .targets
                    .iter()
                    .map(|target| Path::new(self.current).join(target))
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
                      "cwd": path.canonicalize().unwrap(),
                  },
                }])
            }
            _ => {
                let path = dirs[0].as_path();
                let paths = self.path_repository.children(path.to_str().unwrap());
                json!([{
                  "name": "update",
                  "args": paths,
                  "options": {
                      "cwd": path.canonicalize().unwrap(),
                  }
                }])
            }
        }
    }
}
