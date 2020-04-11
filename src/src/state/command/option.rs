#[derive(Debug, Clone, Copy)]
pub struct Split {
    pub name: SplitName,
    pub mod_name: SplitModName,
}

#[derive(Debug, Clone, Copy)]
pub enum SplitName {
    Tab,
    Vertical,
    Horizontal,
    Open,
    No,
    Unknown,
}

impl From<&str> for SplitName {
    fn from(s: &str) -> Self {
        match s {
            "tab" => SplitName::Tab,
            "vertical" => SplitName::Vertical,
            "horizontal" => SplitName::Horizontal,
            "open" => SplitName::Open,
            "no" => SplitName::No,
            _ => SplitName::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SplitModName {
    LeftAbove,
    RightBelow,
    No,
    Unknown,
}

impl From<&str> for SplitModName {
    fn from(s: &str) -> Self {
        match s {
            "leftabove" => SplitModName::LeftAbove,
            "rightbelow" => SplitModName::RightBelow,
            _ => SplitModName::Unknown,
        }
    }
}

impl From<&str> for Split {
    fn from(s: &str) -> Self {
        let names: Vec<_> = s.split(":").collect();
        match &names[..] {
            [name, mod_name] => Split {
                name: SplitName::from(*name),
                mod_name: SplitModName::from(*mod_name),
            },
            [name] => Split {
                name: SplitName::from(*name),
                mod_name: SplitModName::No,
            },
            _ => Split {
                name: SplitName::Unknown,
                mod_name: SplitModName::Unknown,
            },
        }
    }
}

#[derive(Debug)]
pub enum CommandOption {
    Path { value: String },
    Paths { value: String },
    Quit,
    NoConfirm,
    Create,
    Back,
    Split { value: Split },
    Unknown,
}

impl From<&str> for CommandOption {
    fn from(arg: &str) -> Self {
        let key_value: Vec<_> = arg.split("=").collect();
        match &key_value[..] {
            ["quit"] => CommandOption::Quit,
            ["no-confirm"] => CommandOption::NoConfirm,
            ["create"] => CommandOption::Create,
            ["back"] => CommandOption::Back,
            ["path", path] => CommandOption::Path {
                value: path.to_string(),
            },
            ["paths", path] => CommandOption::Paths {
                value: path.to_string(),
            },
            ["split", split] => CommandOption::Split {
                value: Split::from(*split),
            },
            _ => CommandOption::Unknown,
        }
    }
}
