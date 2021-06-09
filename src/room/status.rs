use actix::prelude::Recipient;

use crate::user::User;
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(actix::Message)]
#[rtype(usize)]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<Message>,
    pub user: User,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct DisConnect {
    pub id: usize, // session id
}

#[derive(actix::Message)]
#[rtype(usize)]
pub struct CreateRoom {
    pub name: Option<String>,
    pub sid: usize,
}

#[derive(actix::Message)]
#[rtype(usize)]
pub struct JoinRoom {
    pub rid: usize,
    pub sid: usize,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct SendData {
    pub rid: usize,
    pub msg: String,
    pub sid: usize,
}
