use super::current::Target;
use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;
use itertools::Itertools;

pub struct RemoveCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for RemoveCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let targets = self
            .current
            .targets()
            .into_iter()
            .group_by(|target| target.depth)
            .into_iter()
            .fold(vec![], |mut acc: Vec<Target>, (_, targets)| {
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
            });

        let paths: Vec<_> = targets.iter().map(|target| target.path.clone()).collect();

        if !self.opts.no_confirm {
            return Ok(vec![Action::ConfirmRemove { paths: paths }]);
        }

        self.dispatcher.path_repository().remove(paths)?;

        let mut targets = targets;
        targets.sort_by(|a, b| a.depth.cmp(&b.depth));
        let targets = targets;

        let min_depth = match targets.iter().map(|target| target.depth).min() {
            Some(depth) => depth,
            None => 0,
        };

        targets
            .into_iter()
            .take_while(|target| target.depth <= min_depth)
            .map(|target| {
                let parent_path = match self.dispatcher.path(&target.path).parent() {
                    Some(path) => path,
                    None => self.dispatcher.path_repository().root(),
                };
                (target, parent_path)
            })
            .unique_by(|(_, parent_path)| parent_path.clone())
            .try_fold(vec![], |mut acc, (target, parent_path)| {
                let paths: Paths = match self.dispatcher.path_repository().list(&parent_path) {
                    Ok(ps) => ps.iter().skip(1).collect::<Vec<_>>().into(),
                    Err(err) => return Err(err.into()),
                };

                let action = paths.to_write_action(
                    target.depth as usize,
                    target.parent_id,
                    target.last_sibling_id,
                );
                acc.push(action);
                Ok(acc)
            })
    }
}
