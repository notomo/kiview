use super::action::Paths;
use super::command::CommandResult;
use crate::command::Action;
use crate::command::Command;
use crate::command::Current;
use crate::repository::PathRepository;

pub struct ToggleTreeCommand {
    pub current: Current,
    pub repository: Box<dyn PathRepository>,
}

impl Command for ToggleTreeCommand {
    fn actions(&self) -> CommandResult {
        let actions: Vec<_> = self
            .current
            .dedup_targets(&self.repository, |target| {
                !target.is_parent_node && self.repository.path(&target.path).is_group_node()
            })
            .iter()
            .map(|target| match target.opened {
                true => Action::CloseTree {
                    id: target.id,
                    next_sibling_id: target.next_sibling_id,
                },
                false => match self.repository.children(&target.path) {
                    Ok(children) => {
                        Paths::from(children).to_open_tree_action(target.id, target.depth)
                    }
                    Err(err) => Action::show_error(&target.path, err),
                },
            })
            .collect();
        Ok(actions)
    }
}
