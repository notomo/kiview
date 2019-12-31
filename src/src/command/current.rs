use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Current<'a> {
    pub path: &'a str,

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
    pub fn targets(&self) -> Vec<Target> {
        if self.selected_targets.len() != 0 {
            return self.selected_targets.clone();
        }
        self.targets.clone()
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisteredTarget {
    pub path: String,

    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RenameTarget {
    pub id: u64,
    pub from: String,
    pub to: String,
}
