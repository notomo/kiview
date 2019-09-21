use std::fs;

pub trait PathRepository {
    fn children(&self, path: &str) -> Vec<String>;
}

pub struct FilePathRepository {}

impl PathRepository for FilePathRepository {
    fn children(&self, path: &str) -> Vec<String> {
        let directories: Vec<_> = fs::read_dir(path)
            .unwrap()
            .filter(|path| path.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|path| format!("{}/", path.unwrap().file_name().to_str().unwrap()))
            .collect();

        let files: Vec<_> = fs::read_dir(path)
            .unwrap()
            .filter(|path| !path.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
            .collect();

        [&directories[..], &files[..]].concat()
    }
}
