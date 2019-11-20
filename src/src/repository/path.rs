use std::fs;

pub trait Path {
    fn join_head(&self, head: &str) -> String;
}

pub trait PathRepository<'a> {
    fn list(&self, path: &str) -> Result<Vec<FullPath>, crate::repository::Error>;
    fn path<'b: 'a>(&self, path: &'b str) -> Box<(dyn Path + 'a)>;
}

pub struct FilePathRepository {}

impl<'a> PathRepository<'a> for FilePathRepository {
    fn list(&self, path: &str) -> Result<Vec<FullPath>, crate::repository::Error> {
        let parent_directory = vec![FullPath {
            name: String::from(".."),
            path: std::path::Path::new(&path)
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

    fn path<'b: 'a>(&self, path: &'b str) -> Box<(dyn Path + 'a)> {
        box FilePath {
            path: std::path::Path::new(path),
        }
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

pub struct FilePath<'a> {
    path: &'a std::path::Path,
}

impl<'a> Path for FilePath<'a> {
    fn join_head(&self, head: &str) -> String {
        self.path
            .join(head)
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}
