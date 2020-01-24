use super::command::CommandResult;
use crate::command::Command;
use crate::command::ErrorKind;

pub struct UnknownCommand<'a> {
    pub command_name: &'a str,
}

impl<'a> Command for UnknownCommand<'a> {
    fn actions(&self) -> CommandResult {
        Err(ErrorKind::Unknown {
            command_name: self.command_name.to_string(),
        }
        .into())
    }
}
