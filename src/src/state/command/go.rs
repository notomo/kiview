use async_trait::async_trait;

use crate::repository::PathRepository;
use crate::state::command::option::{CommandOption, Split, SplitModName, SplitName};
use crate::state::command::Command;
use crate::state::CommandResult;
use crate::state::Current;
use crate::state::State;

pub struct GoCommandOptions {
    back: bool,
    create: bool,
    path: Option<String>,
    split: Split,
}

impl From<Vec<CommandOption>> for GoCommandOptions {
    fn from(opts: Vec<CommandOption>) -> Self {
        let mut split = Split {
            name: SplitName::Vertical,
            mod_name: SplitModName::LeftAbove,
        };
        let mut back = false;
        let mut create = false;
        let mut path = None;

        opts.into_iter().for_each(|opt| match opt {
            CommandOption::Back => back = true,
            CommandOption::Create => create = true,
            CommandOption::Path { value } => path = Some(value),
            CommandOption::Split { value } => split = value,
            _ => (),
        });

        GoCommandOptions {
            split: split,
            back: back,
            create: create,
            path: path,
        }
    }
}

pub struct GoCommand {
    pub current: Current,
    pub repository: Box<dyn PathRepository>,
    pub opts: GoCommandOptions,
}

#[async_trait]
impl Command for GoCommand {
    async fn execute(&self, state: &mut State) -> CommandResult {
        let target_group_path = match &self.opts.path {
            Some(opt_path) => self.repository.path(opt_path.as_str()).canonicalize()?,
            None => self
                .repository
                .path(&self.current.path())
                .parent_if_not_exists()?,
        };

        state.cd(&target_group_path).await?;

        if !self
            .repository
            .path(&self.current.path())
            .equals(&target_group_path)
        {
            state.try_to_restore_cursor(&target_group_path).await?;
            state.add_history(
                &self.current.path(),
                self.current.line_number(),
                self.opts.back,
            );
        }

        Ok(())
    }
}
