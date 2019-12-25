use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;

pub struct ToggleTreeCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for ToggleTreeCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let target = match &self.current.target {
            Some(target)
                if !target.is_parent_node && self.dispatcher.path(&target.path).is_group_node() =>
            {
                target
            }
            _ => return Ok(vec![]),
        };

        if target.opened {
            return Ok(vec![Action::CloseTree {
                id: target.id,
                count: (self.current.next_sibling_line_number - self.current.line_number - 1)
                    as usize,
            }]);
        }

        let child_paths: Paths = self
            .dispatcher
            .path_repository()
            .list(&target.path)?
            .iter()
            .skip(1)
            .collect::<Vec<_>>()
            .into();

        Ok(vec![
            child_paths.to_open_tree_action(target.id, target.depth as usize)
        ])
    }
}
