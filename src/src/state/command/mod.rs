use async_trait::async_trait;

mod name;
use name::CommandName;

pub mod option;
use option::CommandOption;

mod go;
use go::GoCommand;

mod error;
use error::Error;

use crate::repository::Dispatcher;
use crate::state::Current;
use crate::state::State;

#[async_trait]
pub trait Command: Send + Sync {
    async fn execute(&self, state: &mut State) -> CommandResult;
}

pub type CommandResult = Result<(), Error>;

pub async fn execute(arg: &str, current: Current, state: &mut State) -> CommandResult {
    let command_name = CommandName::from(arg);
    debug!("{:?}", command_name);

    let opts: Vec<CommandOption> = arg
        .split_whitespace()
        .filter(|arg| arg.starts_with("-"))
        .map(|arg| {
            arg.chars()
                .into_iter()
                .skip(1)
                .collect::<String>()
                .as_str()
                .into()
        })
        .collect();

    let dispatcher = Dispatcher {};
    let path_repository = dispatcher.path_repository();

    match &command_name {
        CommandName::Go => box GoCommand {
            current: current,
            repository: path_repository,
            opts: opts.into(),
        } as Box<dyn Command>,
        _ => return Ok(()),
    }
    .execute(state)
    .await
}
