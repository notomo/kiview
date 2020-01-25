use super::action::Paths;
use super::command::CommandResult;
use super::command::{CommandOption, Split, SplitModName, SplitName};
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::repository::PathRepository;

pub struct NewCommandOptions {
    paths: Vec<String>,
    open: Split,
}

impl From<Vec<CommandOption>> for NewCommandOptions {
    fn from(opts: Vec<CommandOption>) -> Self {
        let mut open = Split {
            name: SplitName::No,
            mod_name: SplitModName::No,
        };
        let mut paths = vec![];
        opts.into_iter().for_each(|opt| match opt {
            CommandOption::Open { value } => open = value,
            CommandOption::Paths { value } => paths.push(value),
            _ => (),
        });
        NewCommandOptions {
            open: open,
            paths: paths,
        }
    }
}

pub struct NewCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: NewCommandOptions,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> CommandResult {
        if self.opts.paths.len() == 0 {
            return Ok(vec![Action::ConfirmNew]);
        };

        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .repository
                .path(&target.path)
                .parent()
                .unwrap_or_else(|| self.current.path.to_string()),
            Some(_) | None => self.current.path.to_string(),
        };

        let (open_paths, errors) =
            self.opts
                .paths
                .iter()
                .fold((vec![], vec![]), |(mut open_paths, mut errors), path| {
                    match self.repository.create_with(&target_group_path, &path) {
                        Ok(_) => {
                            // HACK: not supported group node open
                            if !self.repository.path(&path).is_group_node() {
                                open_paths.push(path.to_string());
                            };
                        }
                        Err(err) => errors.push(Action::ShowError {
                            path: path.to_string(),
                            message: err.inner.to_string(),
                        }),
                    };
                    (open_paths, errors)
                });

        let paths: Paths = self.repository.children(&target_group_path)?.into();

        let actions: Vec<_> = vec![paths.to_write_action(
            match &self.current.target {
                Some(target) => target.depth,
                None => 0,
            },
            self.current.target.as_ref().and_then(|t| t.parent_id),
            self.current.target.as_ref().and_then(|t| t.last_sibling_id),
        )]
        .into_iter()
        .chain(self.opts.open.leaf_node_action(open_paths))
        .chain(errors)
        .collect();

        Ok(actions)
    }
}
