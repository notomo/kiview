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

        let paths: Vec<_> = fs::read_dir(path)?
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap().path())
            .collect();

        let directories: Vec<_> = paths
            .iter()
            .filter(|p| p.is_dir())
            .map(|p| FullPath {
                name: p
                    .file_name()
                    .and_then(|name| name.to_str().and_then(|name| Some(format!("{}/", name))))
                    .unwrap_or(String::from("")),
                path: p
                    .canonicalize()
                    .and_then(|p| Ok(p.to_str().unwrap_or("").to_string()))
                    .unwrap_or(String::from("")),
            })
            .filter(|p| p.name != "" && p.path != "")
            .collect();

        let files: Vec<_> = paths
            .iter()
            .filter(|p| !p.is_dir())
            .map(|p| FullPath {
                name: p
                    .file_name()
                    .and_then(|name| name.to_str().and_then(|name| Some(String::from(name))))
                    .unwrap_or(String::from("")),
                path: p
                    .canonicalize()
                    .and_then(|p| Ok(p.to_str().unwrap_or("").to_string()))
                    .unwrap_or(String::from("")),
            })
            .filter(|p| p.name != "" && p.path != "")
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
