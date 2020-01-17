use super::current::Target;
use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::repository::Dispatcher;
use crate::repository::PathRepository;
use itertools::Itertools;

pub struct ToggleTreeCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub path_repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for ToggleTreeCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let results: Vec<_> = self
            .current
            .targets()
            .filter(|target| {
                !target.is_parent_node && self.dispatcher.path(&target.path).is_group_node()
            })
            .group_by(|target| target.depth)
            .into_iter()
            .fold(vec![], |mut acc: Vec<&Target>, (_, targets)| {
                let mut child_acc: Vec<_> = vec![];
                for target in targets {
                    let count = acc
                        .iter()
                        .filter(|x| self.dispatcher.path(&target.path).contained(&x.path))
                        .count();
                    if count == 0 {
                        child_acc.push(target)
                    }
                }
                acc.extend(child_acc);
                acc
            })
            .iter()
            .map(|target| match target.opened {
                true => Ok(Action::CloseTree {
                    id: target.id,
                    next_sibling_id: target.next_sibling_id,
                }),
                false => {
                    let child_paths: Paths = match self.path_repository.children(&target.path) {
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
