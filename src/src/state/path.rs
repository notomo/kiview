use crate::repository::FullPath;

#[derive(Debug)]
pub struct Paths {
    paths: Vec<FullPath>,
}

impl From<Box<dyn Iterator<Item = FullPath>>> for Paths {
    fn from(paths: Box<dyn Iterator<Item = FullPath>>) -> Paths {
        Paths {
            paths: paths.collect(),
        }
    }
}

impl Paths {
    pub fn lines(&self, depth: u64) -> Vec<String> {
        let indent = std::iter::repeat(" ")
            .take(depth as usize)
            .collect::<String>();
        self.paths
            .iter()
            .map(|p| format!("{}{}", indent, p.name))
            .collect()
    }
}
