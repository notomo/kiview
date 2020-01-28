use super::command::Split;
use crate::command::Error;
use crate::repository::FullPath;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "name")]
pub enum Action {
    #[serde(rename = "open_leaves")]
    OpenLeaves {
        paths: Vec<String>,
        #[serde(flatten)]
        split: Split,
    },

    #[serde(rename = "open_view")]
    OpenView {
        path: String,
        #[serde(flatten)]
        split: Split,
    },
    #[serde(rename = "quit")]
    Quit,

    #[serde(rename = "confirm_remove")]
    ConfirmRemove { paths: Vec<String> },
    #[serde(rename = "confirm_rename")]
    ConfirmRename { path: String, relative_path: String },
    #[serde(rename = "confirm_new")]
    ConfirmNew,

    #[serde(rename = "copy")]
    Copy { items: Vec<RegisteredItem> },
    #[serde(rename = "cut")]
    Cut { items: Vec<RegisteredItem> },
    #[serde(rename = "clear_register")]
    ClearRegister,
    #[serde(rename = "choose")]
    Choose {
        path: String,
        items: Vec<ChooseItem>,
        has_cut: bool,
    },

    #[serde(rename = "select")]
    Select { ids: Vec<u64> },
    #[serde(rename = "unselect")]
    Unselect { ids: Vec<u64> },
    #[serde(rename = "toggle_selection")]
    ToggleSelection { ids: Vec<u64> },
    #[serde(rename = "toggle_all_selection")]
    ToggleAllSelection,

    #[serde(rename = "back_history")]
    BackHistory,
    #[serde(rename = "add_history")]
    AddHistory {
        path: String,
        line_number: u64,
        #[serde(default)]
        back: bool,
    },
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
    #[serde(rename = "write")]
    Write {
        lines: Vec<String>,
        props: Vec<Prop>,
        parent_id: Option<u64>,
        last_sibling_id: Option<u64>,
        count: usize,
    },

    #[serde(rename = "open_tree")]
    OpenTree {
        lines: Vec<String>,
        props: Vec<Prop>,
        id: u64,
        count: usize,
    },
    #[serde(rename = "close_tree")]
    CloseTree {
        id: u64,
        next_sibling_id: Option<u64>,
    },

    #[serde(rename = "fork_buffer")]
    ForkBuffer {
        items: Vec<ForkBufferItem>,
        #[serde(flatten)]
        split: Split,
    },
    #[serde(rename = "show_error")]
    ShowError { path: String, message: String },

    #[serde(rename = "open_renamer")]
    OpenRenamer {
        path: String,
        items: Vec<RenameItem>,
    },
    #[serde(rename = "complete_renamer")]
    CompleteRenamer {
        items: Vec<RenameItem>,
        has_error: bool,
    },
}

impl Action {
    pub fn show_error(path: &str, err: impl Into<Error>) -> Action {
        Action::ShowError {
            path: path.to_string(),
            message: err.into().inner.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ForkBufferItem {
    path: String,
    lines: Vec<String>,
    props: Vec<Prop>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RenameItem {
    pub id: u64,
    pub path: String,
    pub relative_path: String,
    pub is_copy: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Prop {
    path: String,
    depth: u64,
    is_parent_node: bool,
    parent_id: Option<u64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RegisteredItem {
    pub path: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChooseItem {
    pub from: String,
    pub path: String,
    pub relative_path: String,
}

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
    const INDENT_DEPTH: u64 = 2;

    pub fn to_open_tree_action(&self, id: u64, current_depth: u64) -> Action {
        let depth = current_depth + Self::INDENT_DEPTH;
        let parent_id = Some(id);
        let lines = self.lines(depth);
        Action::OpenTree {
            id: id,
            count: (&lines).len(),
            lines: lines,
            props: self.props(depth, parent_id),
        }
    }

    pub fn to_write_all_action(&self) -> Action {
        let depth = 0;
        let parent_id = None;
        Action::WriteAll {
            lines: self.lines(depth),
            props: self.props(depth, parent_id),
        }
    }

    pub fn to_write_action(
        &self,
        depth: u64,
        parent_id: Option<u64>,
        last_sibling_id: Option<u64>,
    ) -> Action {
        let lines = self.lines(depth);
        Action::Write {
            count: (&lines).len(),
            parent_id: parent_id,
            last_sibling_id: last_sibling_id,
            lines: lines,
            props: self.props(depth, parent_id),
        }
    }

    pub fn to_fork_buffer_item(&self, path: &str) -> ForkBufferItem {
        let depth = 0;
        let parent_id = None;
        ForkBufferItem {
            path: path.to_string(),
            lines: self.lines(depth),
            props: self.props(depth, parent_id),
        }
    }

    /// returns 1-based indicies number found by f()
    pub fn search(&self, f: impl Fn(&FullPath) -> bool) -> Option<u64> {
        self.paths
            .iter()
            .enumerate()
            .filter(|(_, p)| f(p))
            .map(|(index, _)| (index + 1) as u64)
            .collect::<Vec<_>>()
            .get(0)
            .and_then(|n| Some(*n))
    }

    fn lines(&self, depth: u64) -> Vec<String> {
        let indent = std::iter::repeat(" ")
            .take(depth as usize)
            .collect::<String>();
        self.paths
            .iter()
            .map(|p| format!("{}{}", indent, p.name))
            .collect()
    }

    fn props(&self, depth: u64, parent_id: Option<u64>) -> Vec<Prop> {
        self.paths
            .iter()
            .map(|p| Prop {
                path: p.path.clone(),
                depth: depth,
                is_parent_node: p.is_parent_node,
                parent_id: parent_id,
            })
            .collect()
    }
}
