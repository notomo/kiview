use crate::repository::{Error, ErrorKind};
use crate::repository::{FullPath, Path, PathRepository};
use path_slash::PathExt;
use std::fs;
use std::path::Path as StdPath;

extern crate fs_extra;
extern crate path_slash;
extern crate url;

pub struct FilePathRepository {}

impl PathRepository for FilePathRepository {
    fn children(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error> {
        let mut paths: Vec<_> = fs::read_dir(path)?
            .filter_map(|result| result.ok())
            .map(|entry| entry.path())
            .collect();
        paths.sort_by(|a, b| b.is_dir().cmp(&a.is_dir()).then(a.cmp(b)));
        let paths = paths;

        let dirs_and_files: Result<Vec<FullPath>, Error> = paths
            .into_iter()
            .map(|p| match p.is_dir() {
                false => (p, ""),
                true => (p, "/"),
            })
            .try_fold(vec![], |mut full_paths, (p, suffix)| {
                let abs_path = url::Url::from_file_path(p.canonicalize()?)?
                    .to_file_path()?
                    .to_slash()?;
                full_paths.push(FullPath {
                    name: format!("{}{}", p.file_name()?.to_str()?, suffix),
                    path: abs_path,
                    is_parent_node: false,
                });
                Ok(full_paths)
            });

        Ok(box dirs_and_files?.into_iter())
    }

    fn list(&self, path: &str) -> Result<Box<dyn Iterator<Item = FullPath>>, Error> {
        let parent = StdPath::new(path)
            .parent()
            .unwrap_or_else(|| StdPath::new(path));
        let abs_path = url::Url::from_file_path(parent.canonicalize()?)?
            .to_file_path()?
            .to_slash()?;

        Ok(box vec![FullPath {
            name: String::from(".."),
            path: abs_path,
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

    fn rename(&self, from: &str, to: &str, force: bool) -> Result<(), Error> {
        if !force && StdPath::new(to).exists() {
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

    fn path<'a>(&self, path: &'a str) -> Box<dyn Path + 'a> {
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
        self.path.parent().and_then(|p| p.to_slash())
    }

    fn canonicalize(&self) -> Result<String, Error> {
        let abs_path = url::Url::from_file_path(self.path.to_path_buf().canonicalize()?)?
            .to_file_path()?
            .to_slash()?;
        Ok(abs_path)
    }

    fn join(&self, path: &str) -> Result<String, Error> {
        Ok(self.path.join(path).to_slash()?)
    }

    fn parent_if_not_exists(&self) -> Result<String, Error> {
        let mut path = self.path;
        while !path.exists() {
            path = path.parent()?;
        }
        let abs_path = url::Url::from_file_path(path.to_path_buf().canonicalize()?)?
            .to_file_path()?
            .to_slash()?;
        Ok(abs_path)
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
        Ok(self.path.to_slash()?)
    }

    fn contained(&self, haystack: &str) -> bool {
        self.path.starts_with(haystack)
    }

    fn to_relative(&self, base: &str) -> Result<String, Error> {
        Ok(self.path.strip_prefix(base)?.to_slash()?)
    }

    fn root(&self) -> String {
        String::from("/")
    }
}
