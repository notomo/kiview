use crate::repository::Error;
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
    fn list(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error>;
    fn children(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error>;
    fn create(&self, path: &str) -> Result<(), Error>;
    fn rename(&self, from: &str, to: &str) -> Result<(), Error>;
    fn copy(&self, from: &str, to: &str) -> Result<(), Error>;
    fn remove(&self, paths: Vec<String>) -> Result<(), Error>;
    fn root(&self) -> String;
    fn new_path<'a>(&self, path: &'a str) -> Box<dyn Path + 'a>;

    fn rename_or_copy(&self, from: &str, to: &str, is_copy: bool) -> Result<(), Error> {
        if is_copy {
            return self.copy(from, to);
        }
        self.rename(from, to)
    }

    fn rename_or_copy_with(
        &self,
        from: &str,
        to: &str,
        joined: &str,
        is_copy: bool,
    ) -> Result<String, Error> {
        let new_path = self.new_path(to).join(joined)?;
        self.rename_or_copy(from, &new_path, is_copy)?;
        Ok(new_path)
    }

    fn create_with<'a>(&self, base_path: &'a str, joined: &'a str) -> Result<(), Error> {
        let new_path = self.new_path(base_path).join(joined)?;
        Ok(self.create(&new_path)?)
    }
}

pub trait Path {
    fn is_group_node(&self) -> bool;
    fn parent(&self) -> Option<String>;
    fn canonicalize(&self) -> Result<String, Error>;
    fn join(&self, path: &str) -> Result<String, Error>;
    fn exists(&self) -> bool;
    fn name(&self) -> Option<String>;
    fn to_string(&self) -> Result<String, Error>;
    fn contained(&self, haystack: &str) -> bool;
    fn to_relative(&self, base: &str) -> Result<String, Error>;
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
