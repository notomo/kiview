use crate::command::Action;
use crate::command::Command;

pub struct UnknownCommand<'a> {
    pub command_name: &'a str,
}

impl<'a> Command for UnknownCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        Err(crate::command::ErrorKind::Unknown {
            command_name: self.command_name.to_string(),
        }
        .into())
    }
}
