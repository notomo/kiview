use crate::repository::Error;
use crate::repository::FilePathRepository;

pub struct Dispatcher {}

impl Dispatcher {
    pub fn path_repository(&self) -> Box<dyn PathRepository> {
        box FilePathRepository {}
    }
}

pub trait PathRepository: Send + Sync {
    fn list(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error>;
    fn children(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error>;
    fn create_leaf(&self, path: &str) -> Result<(), Error>;
    fn create_group(&self, path: &str) -> Result<(), Error>;
    fn rename(&self, from: &str, to: &str, force: bool) -> Result<(), Error>;
    fn copy(&self, from: &str, to: &str) -> Result<(), Error>;
    fn remove(&self, paths: Vec<String>) -> Result<(), Error>;
    fn path<'a>(&self, path: &'a str) -> Box<dyn Path + 'a>;

    fn rename_with(
        &self,
        from: &str,
        base_path: &str,
        joined: &str,
        force: bool,
    ) -> Result<String, Error> {
        let new_path = self.path(base_path).join(joined)?;
        self.rename(from, &new_path, force)?;
        Ok(new_path)
    }

    fn rename_or_copy(
        &self,
        from: &str,
        to: &str,
        is_copy: bool,
        force: bool,
    ) -> Result<(), Error> {
        if is_copy {
            return self.copy(from, to);
        }
        self.rename(from, to, force)
    }

    fn rename_or_copy_with(
        &self,
        from: &str,
        to: &str,
        joined: &str,
        is_copy: bool,
        force: bool,
    ) -> Result<String, Error> {
        let new_path = self.path(to).join(joined)?;
        self.rename_or_copy(from, &new_path, is_copy, force)?;
        Ok(new_path)
    }

    fn create_with<'a>(&self, base_path: &'a str, joined: &'a str) -> Result<String, Error> {
        let new_path = self.path(base_path).join(joined)?;
        match joined.ends_with("/") {
            false => self.create_leaf(&new_path),
            true => self.create_group(&new_path),
        }?;
        Ok(new_path)
    }
}

pub trait Path {
    fn is_group_node(&self) -> bool;
    fn parent(&self) -> Option<String>;
    fn equals(&self, path: &str) -> bool;
    fn canonicalize(&self) -> Result<String, Error>;
    fn join(&self, path: &str) -> Result<String, Error>;
    fn exists(&self) -> bool;
    fn name(&self) -> Option<String>;
    fn to_string(&self) -> Result<String, Error>;
    fn contained(&self, haystack: &str) -> bool;
    fn to_relative(&self, base: &str) -> Result<String, Error>;
    fn root(&self) -> String;
    fn parent_if_not_exists(&self) -> Result<String, Error>;

    fn parent_or_root(&self) -> String {
        match self.parent() {
            Some(path) => path,
            None => self.root(),
        }
    }
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
