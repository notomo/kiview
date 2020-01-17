use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::Dispatcher;
use crate::repository::PathRepository;

pub struct NewCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub path_repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        if self.opts.paths.len() == 0 {
            return Ok(vec![Action::ConfirmNew]);
        };

        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            Some(_) | None => self.current.path.to_string(),
        };
        let depth = match &self.current.target {
            Some(target) => target.depth,
            None => 0,
        };

        let errors: Vec<_> = self
            .opts
            .paths
            .iter()
            .map(
                |p| match self.dispatcher.path(&target_group_path).join(&p) {
                    Ok(new_path) => self.path_repository.create(&new_path),
                    Err(err) => Err(err),
                },
            )
            .filter_map(|res| match res {
                Err(err) => Some(Action::ShowError {
                    path: String::from(""),
                    message: err.inner.to_string(),
                }),
                _ => None,
            })
            .collect();

        let paths: Paths = self.path_repository.children(&target_group_path)?.into();

        let mut actions = vec![paths.to_write_action(
            depth as usize,
            self.current.target.as_ref().and_then(|t| t.parent_id),
            self.current.target.as_ref().and_then(|t| t.last_sibling_id),
        )];
        actions.extend(errors);

        Ok(actions)
    }
}
