use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::repository::PathRepository;

pub struct ToggleTreeCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for ToggleTreeCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let results: Vec<_> = self
            .current
            .dedup_targets(&self.repository, |target| {
                !target.is_parent_node && self.repository.new_path(&target.path).is_group_node()
            })
            .iter()
            .map(|target| match target.opened {
                true => Ok(Action::CloseTree {
                    id: target.id,
                    next_sibling_id: target.next_sibling_id,
                }),
                false => {
                    let child_paths: Paths = match self.repository.children(&target.path) {
                        Ok(paths) => paths.into(),
                        Err(err) => return Err(Error::from(err)),
                    };
                    Ok(child_paths.to_open_tree_action(target.id, target.depth as usize))
                }
            })
            .collect();

        let errors: Vec<_> = results
            .iter()
            .filter_map(|result| result.as_ref().err())
            .collect();
        if errors.len() != 0 {
            return Err(Error::from(ErrorKind::IO {
                message: errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            }));
        }

        let actions: Vec<_> = results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect();

        Ok(actions)
    }
}
