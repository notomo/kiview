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
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "quit")]
    Quit,
    #[serde(rename = "add_history")]
    AddHistory { path: String, line_number: u64 },
    #[serde(rename = "restore_cursor")]
    RestoreCursor {
        path: String,
        line_number: Option<u64>,
    },
    #[serde(rename = "write")]
    Write { paths: Vec<String> },
}
