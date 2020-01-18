use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Error;
use crate::command::Paths;
use crate::repository::PathRepository;

pub struct NewCommand<'a> {
    pub current: Current<'a>,
    pub repository: Box<dyn PathRepository>,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for NewCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        if self.opts.paths.len() == 0 {
            return Ok(vec![Action::ConfirmNew]);
        };

        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .repository
                .new_path(&target.path)
                .parent()
                .unwrap_or_else(|| self.current.path.to_string()),
            Some(_) | None => self.current.path.to_string(),
        };

        let results: Vec<_> = self
            .opts
            .paths
            .iter()
            .map(|path| self.repository.create_with(&target_group_path, &path))
            .collect();

        let paths: Paths = self.repository.children(&target_group_path)?.into();

        let actions: Vec<_> = vec![paths.to_write_action(
            match &self.current.target {
                Some(target) => target.depth,
                None => 0,
            },
            self.current.target.as_ref().and_then(|t| t.parent_id),
            self.current.target.as_ref().and_then(|t| t.last_sibling_id),
        )]
        .into_iter()
        .chain(
            results
                .into_iter()
                .filter_map(|res| Action::show_error(res)),
        )
        .collect();

        Ok(actions)
    }
}
