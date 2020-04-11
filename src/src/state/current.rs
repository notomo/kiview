use crate::state::Vim;

pub struct Current {
    vim: Vim,
}

impl Current {
    pub fn new(vim: Vim) -> Current {
        Current { vim: vim }
    }

    pub fn path(&self) -> String {
        "".into()
    }

    pub fn line_number(&self) -> u64 {
        1
    }
}
