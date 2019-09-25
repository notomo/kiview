use std::fs;

pub trait PathRepository {
    fn list(&self, path: &str) -> Vec<String>;
}

pub struct FilePathRepository {}

impl PathRepository for FilePathRepository {
    fn list(&self, path: &str) -> Vec<String> {
        let parent_directory = vec!["..".to_string()];

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

        [&parent_directory[..], &directories[..], &files[..]].concat()
    }
}
