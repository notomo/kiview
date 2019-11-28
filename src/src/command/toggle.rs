use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct ToggleTreeCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for ToggleTreeCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        if self.current.line_number == 1 {
            return Ok(vec![]);
        }

        if self.current.opened && self.current.next_sibling_line_number > self.current.line_number {
            return Ok(vec![Action::CloseTree {
                root: self.current.line_number as usize,
                count: (self.current.next_sibling_line_number - self.current.line_number - 1)
                    as usize,
            }]);
        }

        match &self.current.target {
            Some(current_target) if self.dispatcher.path(&current_target.path).is_group_node() => {
                let child_paths: Paths = self
                    .dispatcher
                    .path_repository()
                    .list(&current_target.path)?
                    .iter()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .into();

                Ok(vec![child_paths.to_open_tree_action(
                    self.current.line_number as usize,
                    self.current.depth as usize,
                )])
            }
            _ => Ok(vec![]),
        }
    }
}
