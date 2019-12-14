use crate::command::Action;
use crate::command::Command;
use crate::command::{Error, ErrorKind};

pub struct UnknownCommand<'a> {
    pub command_name: &'a str,
}

impl<'a> Command for UnknownCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        Err(ErrorKind::Unknown {
            command_name: self.command_name.to_string(),
        }
        .into())
    }
}
