use super::action::Paths;
use super::command::CommandOption;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::repository::PathRepository;
use itertools::Itertools;

pub struct RemoveCommandOptions {
    no_confirm: bool,
}

impl From<Vec<CommandOption>> for RemoveCommandOptions {
    fn from(opts: Vec<CommandOption>) -> Self {
        let mut no_confirm = false;

        opts.into_iter().for_each(|opt| match opt {
            CommandOption::NoConfirm => no_confirm = true,
            _ => (),
        });

        RemoveCommandOptions {
            no_confirm: no_confirm,
        }
    }
}

pub struct RemoveCommand {
    pub current: Current,
    pub repository: Box<dyn PathRepository>,
    pub opts: RemoveCommandOptions,
}

impl Command for RemoveCommand {
    fn actions(&self) -> CommandResult {
        let targets = self
            .current
            .dedup_targets(&self.repository, |target| !target.is_parent_node);
        let paths: Vec<_> = targets.iter().map(|target| target.path.clone()).collect();

        if !self.opts.no_confirm {
            return Ok(vec![Action::ConfirmRemove { paths: paths }]);
        }

        self.repository.remove(paths)?;

        // for break by take_while()
        let mut targets = targets;
        targets.sort_by(|a, b| a.depth.cmp(&b.depth));
        let targets = targets;
        let min_depth = targets.iter().map(|target| target.depth).min().unwrap_or(0);

        let actions: Vec<_> = targets
            .into_iter()
            .take_while(|target| target.depth == min_depth)
            .map(|target| {
                let parent_path = self.repository.path(&target.path).parent_or_root();
                (target, parent_path)
            })
            .unique_by(|(_, parent_path)| parent_path.clone())
            .map(
                |(target, parent_path)| match self.repository.children(&parent_path) {
                    Ok(children) => Paths::from(children).to_write_action(
                        target.depth,
                        target.parent_id,
                        target.last_sibling_id,
                    ),
                    Err(err) => Action::ShowError {
                        path: parent_path,
                        message: err.inner.to_string(),
                    },
                },
            )
            .collect();
        Ok(actions)
    }
}
