use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Paths;
use crate::repository::{Dispatcher, PathRepository};

pub struct ToggleTreeCommand<'a> {
    pub current_path: &'a str,
    pub line_number: u64,
    pub current_target: Option<&'a str>,
    pub opts: &'a CommandOptions,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
    pub dispatcher: Dispatcher,
    pub next_sibling_line_number: u64,
    pub depth: u64,
}

impl<'a> Command for ToggleTreeCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        if self.line_number == 1 {
            return Ok(vec![]);
        }

        if self.next_sibling_line_number > self.line_number + 1 {
            return Ok(vec![Action::Write {
                props: vec![],
                lines: vec![],
                start: self.line_number as usize,
                end: (self.next_sibling_line_number - 1) as usize,
            }]);
        }

        match self.current_target {
            Some(current_target) if self.dispatcher.path(current_target).is_group_node() => {
                let child_paths: Paths = self
                    .path_repository
                    .list(current_target)?
                    .iter()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .into();

                Ok(vec![child_paths
                    .add_indent(self.depth as usize)
                    .to_write_action(
                        self.line_number as usize,
                        self.line_number as usize,
                    )])
            }
            _ => Ok(vec![]),
        }
    }
}
