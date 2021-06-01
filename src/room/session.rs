pub mod status;
use actix::prelude::{Actor, Context, Handler, Recipient, SendError};
use status;

#[derive(Clone)]
pub struct Session {
    pub id: usize,
    pub addr: Recipient<status::Message>,
    pub user: User,
}

impl Session {
    pub fn send(&self, msg: &str) {}
}
