use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;

pub struct ToggleSelectionCommand {
    pub current: Current,
}

impl Command for ToggleSelectionCommand {
    fn actions(&self) -> CommandResult {
        Ok(vec![Action::ToggleSelection {
            ids: self.current.target_ids(),
        }])
    }
}

pub struct SelectCommand {
    pub current: Current,
}

impl Command for SelectCommand {
    fn actions(&self) -> CommandResult {
        Ok(vec![Action::Select {
            ids: self.current.target_ids(),
        }])
    }
}

pub struct UnselectCommand {
    pub current: Current,
}

impl Command for UnselectCommand {
    fn actions(&self) -> CommandResult {
        Ok(vec![Action::Unselect {
            ids: self.current.target_ids(),
        }])
    }
}

impl Current {
    fn target_ids(&self) -> Vec<u64> {
        self.targets
            .iter()
            .filter(|target| !target.is_parent_node)
            .map(|target| target.id)
            .collect()
    }
}
