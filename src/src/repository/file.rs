use crate::repository::{Error, ErrorKind};
use crate::repository::{FullPath, Path, PathRepository};
use std::fs;
use std::path::Path as StdPath;

extern crate fs_extra;

pub struct FilePathRepository {}

impl PathRepository for FilePathRepository {
    fn children(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error> {
        let mut paths: Vec<_> = fs::read_dir(path)?
            .filter_map(|result| result.ok())
            .map(|entry| entry.path())
            .collect();
        paths.sort_by(|a, b| b.is_dir().cmp(&a.is_dir()));
        let paths = paths;

        let dirs_and_files = paths
            .into_iter()
            .map(|p| match p.is_dir() {
                false => (p, ""),
                true => (p, "/"),
            })
            .map(|(p, suffix)| FullPath {
                name: p
                    .file_name()
                    .and_then(|name| {
                        name.to_str()
                            .and_then(|name| Some(format!("{}{}", name, suffix)))
                    })
                    .unwrap_or(String::from("")),
                path: p
                    .canonicalize()
                    .and_then(|p| Ok(p.to_str().unwrap_or("").to_string()))
                    .unwrap_or(String::from("")),
                is_parent_node: false,
            })
            .filter(|p| p.name != "" && p.path != "");

        Ok(box dirs_and_files)
    }

    fn list(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error> {
        Ok(box vec![FullPath {
            name: String::from(".."),
            path: StdPath::new(path)
                .parent()
                .unwrap_or_else(|| StdPath::new(path))
                .canonicalize()?
                .to_str()?
                .to_string(),
            is_parent_node: true,
        }]
        .into_iter()
        .chain(self.children(path)?))
    }

    fn create(&self, path: &str) -> Result<(), Error> {
        if StdPath::new(path).exists() {
            return Err(ErrorKind::AlreadyExists {
                path: path.to_string(),
            }
            .into());
        }

        Ok(match path.ends_with("/") {
            true => fs::create_dir_all(path).and_then(|_| Ok(())),
            false => fs::File::create(path).and_then(|_| Ok(())),
        }?)
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), Error> {
        if StdPath::new(to).exists() {
            return Err(ErrorKind::AlreadyExists {
                path: to.to_string(),
            }
            .into());
        }

        Ok(fs::rename(from, to)?)
    }

    fn copy(&self, from: &str, to: &str) -> Result<(), Error> {
        if StdPath::new(from).is_dir() {
            let mut options = fs_extra::dir::CopyOptions::new();
            options.copy_inside = true;
            fs_extra::dir::copy(from, to, &options)?;
            return Ok(());
        }

        fs::copy(from, to).and_then(|_| Ok(()))?;
        debug!("file copied: {} to {}", from, to);
        Ok(())
    }

    fn remove(&self, paths: Vec<String>) -> Result<(), Error> {
        let files: Vec<_> = paths
            .iter()
            .filter(|path| !StdPath::new(path).is_dir())
            .collect();
        for file in &files {
            fs::remove_file(file)?;
        }

        let dirs: Vec<_> = paths
            .iter()
            .filter(|path| StdPath::new(path).is_dir())
            .collect();
        for dir in &dirs {
            fs::remove_dir_all(dir)?;
        }

        Ok(())
    }

    fn root(&self) -> String {
        String::from("/")
    }

    fn new_path<'a>(&self, path: &'a str) -> Box<dyn Path + 'a> {
        box FilePath {
            path: std::path::Path::new(path),
        }
    }
}

pub struct FilePath<'a> {
    pub path: &'a StdPath,
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

    fn canonicalize(&self) -> Result<String, Error> {
        let fulll_path = self.path.to_path_buf().canonicalize()?;
        Ok(fulll_path.to_str()?.to_string())
    }

    fn join(&self, path: &str) -> Result<String, Error> {
        Ok(self.path.join(path).to_str()?.to_string())
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn name(&self) -> Option<String> {
        self.path
            .file_name()
            .and_then(|p| p.to_str())
            .and_then(|p| Some(p.to_string()))
    }

    fn to_string(&self) -> Result<String, Error> {
        Ok(self.path.to_str()?.to_string())
    }

    fn contained(&self, haystack: &str) -> bool {
        self.path.starts_with(haystack)
    }

    fn to_relative(&self, base: &str) -> Result<String, Error> {
        Ok(self.path.strip_prefix(base)?.to_str()?.to_string())
    }
}
