use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Current<'a> {
    pub path: &'a str,

    pub line_number: u64,

    #[serde(default)]
    pub next_sibling_line_number: u64,

    #[serde(default)]
    pub last_sibling_line_number: u64,

    #[serde(default)]
    pub created: bool,

    #[serde(default)]
    pub has_cut: bool,

    pub target: Option<Target>,

    #[serde(default)]
    pub targets: Vec<Target>,

    #[serde(default)]
    pub selected_targets: Vec<Target>,

    #[serde(default)]
    pub registered_paths: Vec<&'a str>,
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
    pub depth: u64,
    pub opened: bool,
}

impl Target {
    pub fn to_string(&self) -> String {
        self.path.clone()
    }
}
