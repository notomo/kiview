use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct GoCommand<'a> {
    pub current: Current<'a>,
    pub opts: &'a CommandOptions,
    pub dispatcher: Dispatcher,
}

impl<'a> Command for GoCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let current_path = self
            .dispatcher
            .path(match &self.opts.path {
                Some(opt_path) => opt_path.as_str(),
                None => self.current.path,
            })
            .canonicalize()?;

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(&current_path)?
            .into();

        let mut actions = vec![
            paths.to_write_all_action(),
            Action::TryToRestoreCursor { path: current_path },
            Action::AddHistory {
                path: self.current.path.to_string(),
                line_number: self.current.line_number,
            },
        ];
        if !self.current.created {
            actions.push(Action::Create);
        }

        Ok(actions)
    }
}
