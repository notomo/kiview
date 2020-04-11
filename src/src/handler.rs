use async_trait::async_trait;
use nvim_rs::{compat::tokio::Compat, Handler};
use rmpv::Value;
use tokio::io::Stdout;

use crate::state::{Vim, STATE};

#[derive(Clone)]
pub struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(&self, _name: String, args: Vec<Value>, vim: Vim) {
        let mut state = STATE.lock().await;
        let arg = args[0].as_str().unwrap();
        debug!("{}", arg);
        state.mutate(vim, &arg).await.unwrap();
    }
}
