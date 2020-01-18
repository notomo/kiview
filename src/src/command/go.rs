use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct GoCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for GoCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let current_path = self
            .repository
            .new_path(match &self.opts.path {
                Some(opt_path) => opt_path.as_str(),
                None => self.current.path,
            })
            .canonicalize()?;

        let paths: Paths = self.repository.list(&current_path)?.into();

        if self.current.used && self.opts.create {
            return Ok(vec![Action::ForkBuffer {
                items: vec![paths.to_fork_buffer_item(&current_path)],
                split_name: self.opts.split.name,
                mod_name: self.opts.split.mod_name,
            }]);
        }

        let mut actions = vec![paths.to_write_all_action()];

        if self.current.path != current_path {
            actions.push(Action::TryToRestoreCursor {
                path: current_path.clone(),
            });
            actions.push(Action::AddHistory {
                path: self.current.path.to_string(),
                line_number: self.current.line_number,
                back: self.opts.back,
            });
        }

        if !self.current.used {
            actions.push(Action::Create {
                path: current_path,
                split_name: self.opts.split.name,
                mod_name: self.opts.split.mod_name,
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
