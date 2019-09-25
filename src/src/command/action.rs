use serde_derive::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "name")]
pub enum Action {
    #[serde(rename = "open")]
    Open { args: Vec<String> },
    #[serde(rename = "tab_open")]
    TabOpen { args: Vec<String> },
    #[serde(rename = "vertical_open")]
    VerticalOpen { args: Vec<String> },
    #[serde(rename = "create")]
    Create {
        args: Vec<String>,
        options: ActionOptions,
    },
    #[serde(rename = "confirm_remove")]
    ConfirmRemove {},
    #[serde(rename = "confirm_rename")]
    ConfirmRename { arg: String },
    #[serde(rename = "cut")]
    Cut { args: Vec<String> },
    #[serde(rename = "clear_register")]
    ClearRegister {},
    #[serde(rename = "update")]
    Update {
        args: Vec<String>,
        options: ActionOptions,
    },
    #[serde(rename = "copy")]
    Copy { args: Vec<String> },
    #[serde(rename = "confirm_new")]
    ConfirmNew {},
    #[serde(rename = "unknown")]
    Unknown {},
    #[serde(rename = "quit")]
    Quit {},
}

impl Action {
    pub fn options(
        current_path: Option<String>,
        last_path: Option<String>,
        last_line_number: Option<u64>,
        last_path_line_number: Option<u64>,
    ) -> ActionOptions {
        ActionOptions {
            current_path: current_path,
            last_path: last_path,
            last_line_number: last_line_number,
            last_path_line_number: last_path_line_number,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ActionOptions {
    current_path: Option<String>,
    last_path: Option<String>,
    last_line_number: Option<u64>,
    last_path_line_number: Option<u64>,
}
