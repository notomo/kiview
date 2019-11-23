use std::fs;

pub trait PathRepository<'a> {
    fn list(&self, path: &str) -> Result<Vec<FullPath>, crate::repository::Error>;
}

pub struct FilePathRepository {}

impl<'a> PathRepository<'a> for FilePathRepository {
    fn list(&self, path: &str) -> Result<Vec<FullPath>, crate::repository::Error> {
        let parent_directory = vec![FullPath {
            name: String::from(".."),
            path: std::path::Path::new(&path)
                .parent()
                .unwrap_or_else(|| std::path::Path::new(&path))
                .canonicalize()?
                .to_str()?
                .to_string(),
        }];

        let directories: Vec<_> = fs::read_dir(path)?
            .filter(|p| p.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|p| FullPath {
                name: format!(
                    "{}/",
                    p.as_ref().unwrap().file_name().clone().to_str().unwrap()
                ),
                path: p
                    .as_ref()
                    .unwrap()
                    .path()
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            })
            .collect();

        let files: Vec<_> = fs::read_dir(path)?
            .filter(|path| !path.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|p| FullPath {
                path: p
                    .as_ref()
                    .unwrap()
                    .path()
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                name: format!("{}", p.as_ref().unwrap().file_name().to_str().unwrap()),
            })
            .collect();

        Ok([&parent_directory[..], &directories[..], &files[..]].concat())
    }
}

#[derive(Debug, Clone)]
pub struct FullPath {
    pub name: String,
    pub path: String,
}

impl std::fmt::Display for FullPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.path)
    }
}
