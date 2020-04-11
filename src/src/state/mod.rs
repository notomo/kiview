use nvim_rs::{compat::tokio::Compat, Buffer as VimBuffer, Neovim};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::Stdout;
use tokio::sync::Mutex;

use crate::repository::Dispatcher;
use crate::repository::PathRepository;

mod error;
pub use error::Error;

pub mod command;
use command::execute;
use command::CommandResult;

pub mod current;
pub use current::Current;

pub mod path;
pub use path::Paths;

pub type Vim = Neovim<Compat<Stdout>>;
type Buffer = VimBuffer<Compat<Stdout>>;

lazy_static! {
    pub static ref STATE: Arc<Mutex<PluginState>> = Arc::new(Mutex::new(PluginState::new()));
}

pub struct PluginState {
    states: HashMap<i64, StoredState>,
}

impl PluginState {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub async fn mutate(&mut self, vim: Vim, arg: &str) -> CommandResult {
        let current = Current::new(vim.clone());

        let dispatcher = Dispatcher {};
        let path_repository = dispatcher.path_repository();

        let buffer = vim.get_current_buf().await?;

        let bufnr = &buffer.get_number().await?;
        let mut state = match self.states.get(bufnr) {
            Some(stored) => stored.to_state(vim, buffer, path_repository),
            None => State::new(vim, buffer, path_repository),
        };

        let result = execute(arg, current, &mut state).await;
        self.states.insert(bufnr.clone(), state.to_stored());
        result
    }
}

pub struct StoredState {
    history: Arc<Mutex<Vec<String>>>,
}

impl StoredState {
    pub fn to_state(&self, vim: Vim, buffer: Buffer, repository: Box<dyn PathRepository>) -> State {
        State {
            vim: vim,
            buffer: buffer,
            repository: repository,
            history: self.history.clone(),
        }
    }
}

pub struct State {
    vim: Vim,
    buffer: Buffer,
    repository: Box<dyn PathRepository>,
    history: Arc<Mutex<Vec<String>>>,
}

type MutateResult = Result<(), Error>;

impl State {
    pub fn new(vim: Vim, buffer: Buffer, repository: Box<dyn PathRepository>) -> Self {
        Self {
            vim: vim,
            buffer: buffer,
            repository: repository,
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn to_stored(&self) -> StoredState {
        StoredState {
            history: self.history.clone(),
        }
    }

    pub async fn cd(&mut self, path: &str) -> MutateResult {
        self.render(path).await?;
        let cmd = format!("tcd {}", path);
        Ok(self.vim.command(&cmd).await?)
    }

    pub async fn render(&mut self, path: &str) -> MutateResult {
        let paths: Paths = self.repository.list(&path)?.into();
        Ok(self.buffer.set_lines(0, -1, true, paths.lines(0)).await?)
    }

    pub async fn try_to_restore_cursor(&mut self, path: &str) -> MutateResult {
        let cmd = format!("tcd {}", path);
        Ok(self.vim.command(&cmd).await?)
    }

    pub async fn add_history(&mut self, path: &str, _line_number: u64, _is_back: bool) {
        let mut history = self.history.lock().await;
        history.push(path.to_string());
    }
}
