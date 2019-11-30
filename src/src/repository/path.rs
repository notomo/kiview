use crate::repository::{FilePath, FilePathRepository};

pub struct Dispatcher {}

impl Dispatcher {
    pub fn path<'a>(&self, path: &'a str) -> Box<dyn Path + 'a> {
        box FilePath {
            path: std::path::Path::new(path),
        }
    }

    pub fn path_repository(&self) -> Box<dyn PathRepository> {
        box FilePathRepository {}
    }
}

pub trait PathRepository {
    fn list(&self, path: &str) -> Result<Vec<FullPath>, crate::repository::Error>;
    fn create(&self, path: &str) -> Result<(), crate::repository::Error>;
    fn rename(&self, from: &str, to: &str) -> Result<(), crate::repository::Error>;
    fn remove(&self, paths: Vec<String>) -> Result<(), crate::repository::Error>;
}

pub trait Path {
    fn is_group_node(&self) -> bool;
    fn parent(&self) -> Option<String>;
    fn canonicalize(&self) -> Result<String, crate::repository::Error>;
    fn join(&self, path: &str) -> Result<String, crate::repository::Error>;
    fn to_string(&self) -> Result<String, crate::repository::Error>;
}

#[derive(Debug, Clone)]
pub struct FullPath {
    pub name: String,
    pub path: String,
    pub is_parent_node: bool,
}

impl std::fmt::Display for FullPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.path)
    }
}
