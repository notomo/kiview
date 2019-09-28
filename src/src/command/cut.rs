use crate::command::Action;
use crate::command::Command;
use crate::repository::PathRepository;

pub struct CutCommand<'a> {
    pub current_path: &'a str,
    pub targets: Vec<&'a str>,
    pub path_repository: &'a dyn PathRepository<'a>,
}

impl<'a> Command for CutCommand<'a> {
    fn actions(&self) -> Vec<Action> {
        let path = self.path_repository.path(self.current_path);

        let paths = self
            .targets
            .iter()
            .map(|target| path.join_head(target))
            .collect();

        vec![Action::Cut { paths: paths }]
    }
}
