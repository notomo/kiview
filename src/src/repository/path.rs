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

pub struct Dispatcher {}

impl Dispatcher {
    pub fn path<'a>(&self, path: &'a str) -> Box<dyn Path + 'a> {
        box FilePath {
            path: std::path::Path::new(path),
        }
    }
}

pub trait Path {
    fn is_group_node(&self) -> bool;
    fn parent(&self) -> Option<String>;
    fn canonicalize(&self) -> Result<String, crate::repository::Error>;
}

pub struct FilePath<'a> {
    path: &'a std::path::Path,
}

impl<'a> Path for FilePath<'a> {
    fn is_group_node(&self) -> bool {
        self.path.is_dir()
    }

    fn parent(&self) -> Option<String> {
        self.path
            .parent()
            .and_then(|p| p.to_str())
            .and_then(|p| Some(p.to_string()))
    }

    fn canonicalize(&self) -> Result<String, crate::repository::Error> {
        let fulll_path = self.path.to_path_buf().canonicalize()?;
        Ok(fulll_path.to_str()?.to_string())
    }
}
