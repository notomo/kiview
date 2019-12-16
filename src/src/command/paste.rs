use crate::command::Action;
use crate::command::Command;
use crate::command::CommandOptions;
use crate::command::Current;
use crate::command::Paths;
use crate::command::{Error, ErrorKind};
use crate::itertools::Itertools;
use crate::repository::Dispatcher;

pub struct PasteCommand<'a> {
    pub current: Current<'a>,
    pub dispatcher: Dispatcher,
    pub opts: &'a CommandOptions,
}

impl<'a> Command for PasteCommand<'a> {
    fn actions(&self) -> Result<Vec<Action>, Error> {
        let repository = self.dispatcher.path_repository();
        let target_group_path = match &self.current.target {
            Some(target) if !target.is_parent_node => self
                .dispatcher
                .path(&target.path)
                .parent()
                .unwrap_or(target.path.clone()),
            Some(_) | None => self.current.path.to_string(),
        };

        let from_paths: Vec<_> = self
            .current
            .registered_paths
            .iter()
            .unique()
            .map(|path| self.dispatcher.path(path))
            .collect();

        let pair_results: Vec<_> = from_paths
            .iter()
            .map(|from| match from.name() {
                Some(name) => Ok((from, name)),
                None => Err(Error::from(ErrorKind::IO {
                    message: String::from("name not found"),
                })),
            })
            .map(|res| {
                if res.is_err() {
                    return res;
                }
                let (from, name) = res.unwrap();
                match self.dispatcher.path(&target_group_path).join(&name) {
                    Ok(to) => Ok((from, to)),
                    Err(err) => Err(err.into()),
                }
            })
            .collect();

        let errors: Vec<_> = pair_results
            .iter()
            .filter(|pair| pair.is_err())
            .map(|pair| pair.as_ref().err().unwrap())
            .collect();
        if errors.len() != 0 {
            return Err(Error::from(ErrorKind::IO {
                message: errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            }));
        }

        let pairs: Vec<_> = pair_results
            .iter()
            .filter(|pair| pair.is_ok())
            .map(|pair| pair.as_ref().unwrap())
            .collect();

        let already_exists: Vec<_> = pairs
            .iter()
            .filter(|(_, to)| !self.opts.no_confirm && self.dispatcher.path(&to).exists())
            .collect();

        for (from_path, to) in pairs
            .iter()
            .filter(|(_, to)| self.opts.no_confirm || !self.dispatcher.path(&to).exists())
        {
            let from = from_path.to_string()?;
            match self.current.has_cut {
                true => repository.rename(&from, &to),
                false => repository.copy(&from, &to),
            }?;
        }

        let paths: Paths = self
            .dispatcher
            .path_repository()
            .list(&target_group_path)?
            .iter()
            .skip(1)
            .collect::<Vec<_>>()
            .into();

        Ok(vec![
            paths.to_write_action(
                self.current.depth as usize,
                self.current.parent_line_number as usize,
                self.current.last_sibling_line_number as usize,
            ),
            Action::ClearRegister,
            Action::Choose {
                paths: already_exists
                    .iter()
                    .map(|(_, to)| to.to_string())
                    .collect(),
                has_cut: self.current.has_cut,
            },
        ])
    }
}
