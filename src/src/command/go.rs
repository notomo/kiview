use super::action::Paths;
use super::command::CommandResult;
use super::command::{CommandOption, Split, SplitModName, SplitName};
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::repository::PathRepository;

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

pub struct GoCommand<'a> {
    pub current: &'a Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: GoCommandOptions,
}

impl<'a> Command for GoCommand<'a> {
    fn actions(&self) -> CommandResult {
        let target_group_path = self
            .repository
            .path(match &self.opts.path {
                Some(opt_path) => opt_path.as_str(),
                None => self.current.path,
            })
            .canonicalize()?;

        let paths: Paths = self.repository.list(&target_group_path)?.into();

        if self.current.opened && self.opts.create {
            return Ok(vec![
                paths.to_fork_buffer(&target_group_path, self.opts.split)
            ]);
        }

        let mut actions = vec![paths.to_write_all_action()];

        if self.current.path != target_group_path {
            actions.extend(vec![
                Action::TryToRestoreCursor {
                    path: target_group_path.clone(),
                },
                Action::AddHistory {
                    path: self.current.path.to_string(),
                    line_number: self.current.line_number,
                    back: self.opts.back,
                },
            ]);
        }

        if !self.current.opened {
            actions.push(Action::OpenView {
                path: target_group_path,
                split: self.opts.split,
            });

            if let Some(line_number) = paths.search(|p| p.name == self.current.name) {
                actions.push(Action::SetCursor {
                    line_number: line_number,
                });
            }
        }

        Ok(actions)
    }
}
