use std::fs;

pub trait Path {
    fn join_head(&self, head: &str) -> String;
}

pub trait PathRepository<'a> {
    fn list(&self, path: &str) -> Result<Vec<String>, crate::repository::Error>;
    fn path<'b: 'a>(&self, path: &'b str) -> Box<(dyn Path + 'a)>;
}

pub struct FilePathRepository {}

impl<'a> PathRepository<'a> for FilePathRepository {
    fn list(&self, path: &str) -> Result<Vec<String>, crate::repository::Error> {
        let parent_directory = vec!["..".to_string()];

        let directories: Vec<_> = fs::read_dir(path)?
            .filter(|path| path.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|path| format!("{}/", path.unwrap().file_name().to_str().unwrap()))
            .collect();

        let files: Vec<_> = fs::read_dir(path)?
            .filter(|path| !path.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
            .collect();

        Ok([&parent_directory[..], &directories[..], &files[..]].concat())
    }

    fn path<'b: 'a>(&self, path: &'b str) -> Box<(dyn Path + 'a)> {
        box FilePath {
            path: std::path::Path::new(path),
        }
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
