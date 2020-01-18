use crate::repository::PathRepository;

use itertools::Itertools;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Current<'a> {
    pub path: &'a str,

    pub name: &'a str,

    pub line_number: u64,

    #[serde(default)]
    pub used: bool,

    pub has_cut: bool,
    pub renamer_opened: bool,

    #[serde(default)]
    pub target: Option<Target>,

    #[serde(default)]
    pub targets: Vec<Target>,

    #[serde(default)]
    pub selected_targets: Vec<Target>,

    #[serde(default)]
    pub registered_targets: Vec<RegisteredTarget>,

    #[serde(default)]
    pub rename_targets: Vec<RenameTarget>,
}

impl<'a> Current<'a> {
    pub fn targets(&self) -> impl Iterator<Item = &Target> {
        if self.selected_targets.len() != 0 {
            return self.selected_targets.iter();
        }
        self.targets.iter()
    }

    /// if parent and its children exists, this returns parent target only.
    pub fn dedup_targets(
        &self,
        repository: &Box<dyn PathRepository>,
        f: impl Fn(&Target) -> bool,
    ) -> Vec<&Target> {
        self.targets()
            .filter(|target| f(target))
            .group_by(|target| target.depth)
            .into_iter()
            .fold(vec![], |mut acc: Vec<&Target>, (_, targets)| {
                let mut child_acc: Vec<_> = vec![];
                for target in targets {
                    if !acc
                        .iter()
                        .any(|x| repository.new_path(&target.path).contained(&x.path))
                    {
                        child_acc.push(target)
                    }
                }
                acc.extend(child_acc);
                acc
            })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Target {
    pub id: u64,
    pub path: String,
    pub is_parent_node: bool,
    pub parent_id: Option<u64>,
    pub last_sibling_id: Option<u64>,
    pub next_sibling_id: Option<u64>,
    pub depth: u64,
    pub opened: bool,
}

impl Target {
    pub fn to_string(&self) -> String {
        self.path.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisteredTarget {
    pub path: String,

    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RenameTarget {
    pub id: u64,
    pub from: String,
    pub to: String,

    #[serde(default)]
    pub is_copy: bool,
}
