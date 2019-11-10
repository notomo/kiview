use crate::command::Action;
use crate::command::Command;
use crate::repository::PathRepository;

pub struct CutCommand<'a> {
    pub current_path: &'a str,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, crate::command::Error> {
        let path = self.path_repository.path(self.current_path);

        let paths = self
            .targets
            .iter()
            .map(|target| path.join_head(target))
            .collect();

        Ok(vec![Action::Cut { paths: paths }])
    }
}
