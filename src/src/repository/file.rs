use crate::repository::{FullPath, Path, PathRepository};
use std::fs;

pub struct FilePathRepository {}

impl PathRepository for FilePathRepository {
    fn list(&self, path: &str) -> Result<Vec<FullPath>, crate::repository::Error> {
        let parent_directory = vec![FullPath {
            name: String::from(".."),
            path: std::path::Path::new(&path)
                .parent()
                .unwrap_or_else(|| std::path::Path::new(&path))
                .canonicalize()?
                .to_str()?
                .to_string(),
            is_parent_node: true,
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
                is_parent_node: false,
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
                is_parent_node: false,
            })
            .filter(|p| p.name != "" && p.path != "")
            .collect();

        Ok([&parent_directory[..], &directories[..], &files[..]].concat())
    }

    fn create(&self, path: &str) -> Result<(), crate::repository::Error> {
        Ok(match path.ends_with("/") {
            true => fs::create_dir_all(path).and_then(|_| Ok(())),
            false => fs::File::create(path).and_then(|_| Ok(())),
        }?)
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), crate::repository::Error> {
        Ok(fs::rename(from, to)?)
    }

    fn copy(&self, from: &str, to: &str) -> Result<(), crate::repository::Error> {
        fs::copy(from, to)?;
        Ok(())
    }

    fn remove(&self, paths: Vec<String>) -> Result<(), crate::repository::Error> {
        let files: Vec<_> = paths
            .iter()
            .filter(|path| !std::path::Path::new(path).is_dir())
            .collect();
        for file in &files {
            fs::remove_file(file)?;
        }

        let dirs: Vec<_> = paths
            .iter()
            .filter(|path| std::path::Path::new(path).is_dir())
            .collect();
        for dir in &dirs {
            fs::remove_dir_all(dir)?;
        }

        Ok(())
    }
}

pub struct FilePath<'a> {
    pub path: &'a std::path::Path,
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

    fn join(&self, path: &str) -> Result<String, crate::repository::Error> {
        Ok(self.path.join(path).to_str()?.to_string())
    }

    fn name(&self) -> Option<String> {
        self.path
            .file_name()
            .and_then(|p| p.to_str())
            .and_then(|p| Some(p.to_string()))
    }

    fn to_string(&self) -> Result<String, crate::repository::Error> {
        Ok(self.path.to_str()?.to_string())
    }
}