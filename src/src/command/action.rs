use crate::repository::FullPath;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "name")]
pub enum Action {
    #[serde(rename = "open")]
    Open { paths: Vec<String> },
    #[serde(rename = "tab_open")]
    TabOpen { paths: Vec<String> },
    #[serde(rename = "vertical_open")]
    VerticalOpen { paths: Vec<String> },
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "confirm_remove")]
    ConfirmRemove,
    #[serde(rename = "confirm_rename")]
    ConfirmRename { path: String },
    #[serde(rename = "cut")]
    Cut { paths: Vec<String> },
    #[serde(rename = "clear_register")]
    ClearRegister,
    #[serde(rename = "copy")]
    Copy { paths: Vec<String> },
    #[serde(rename = "confirm_new")]
    ConfirmNew,
    #[serde(rename = "quit")]
    Quit,
    #[serde(rename = "add_history")]
    AddHistory { path: String, line_number: u64 },
    #[serde(rename = "try_to_restore_cursor")]
    TryToRestoreCursor { path: String },
    #[serde(rename = "set_cursor")]
    SetCursor { line_number: u64 },
    #[serde(rename = "set_path")]
    SetPath { path: String },
    #[serde(rename = "write_all")]
    WriteAll {
        lines: Vec<String>,
        props: Vec<Prop>,
    },
    #[serde(rename = "open_tree")]
    OpenTree {
        lines: Vec<String>,
        props: Vec<Prop>,
        root: usize,
        count: usize,
    },
    #[serde(rename = "close_tree")]
    CloseTree { root: usize, count: usize },
}

#[derive(Debug, Serialize)]
pub struct Prop {
    path: String,
    depth: usize,
}

impl From<Vec<FullPath>> for Paths {
    fn from(paths: Vec<FullPath>) -> Paths {
        Paths { paths: paths }
    }
}

impl From<Vec<&FullPath>> for Paths {
    fn from(paths: Vec<&FullPath>) -> Paths {
        Paths {
            paths: paths.into_iter().map(|p| p.clone()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Paths {
    paths: Vec<FullPath>,
}

impl IntoIterator for Paths {
    type Item = FullPath;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

impl Paths {
    pub fn to_open_tree_action(&self, root: usize, current_depth: usize) -> Action {
        let indent = std::iter::repeat(" ")
            .take(current_depth)
            .collect::<String>();
        let lines: Vec<_> = self
            .paths
            .iter()
            .map(|p| format!("{}  {}", indent, p.name))
            .collect();

        let depth = current_depth + 2;

        Action::OpenTree {
            count: (&lines).len(),
            lines: lines,
            props: self
                .paths
                .iter()
                .map(|p| Prop {
                    path: p.path.clone(),
                    depth: depth,
                })
                .collect::<Vec<Prop>>(),
            root: root,
        }
    }

    pub fn to_write_all_action(&self) -> Action {
        Action::WriteAll {
            lines: self.paths.iter().map(|p| p.name.clone()).collect(),
            props: self
                .paths
                .iter()
                .map(|p| Prop {
                    path: p.path.clone(),
                    depth: 0 as usize,
                })
                .collect::<Vec<Prop>>(),
        }
    }
}
