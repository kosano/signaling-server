use crate::user::User;
use actix::prelude::{Actor, Context, Handler, Recipient, SendError};
use actix_web::guard::Options;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::HashMap;
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct Connect {
    pub uid: usize,
    pub addr: Recipient<Message>,
}

impl actix::Message for Connect {
    type Result = usize;
}

pub struct DisConnect {
    pub id: usize,
}

impl actix::Message for DisConnect {
    type Result = ();
}
#[derive(Clone, Debug)]
pub struct Session {
    pub id: usize,
    pub addr: Recipient<Message>,
    pub user: User,
}

impl Session {
    pub fn new(id: usize, addr: Recipient<Message>, user: User) -> Self {
        Session {
            id: rand::thread_rng().gen::<usize>(),
            addr: addr,
            user: user,
        }
    }

    pub fn send_message(&self, message: &str) {
        self.addr
            .do_send(Message(message.to_owned()))
            .expect("send error.");
    }
}

impl Actor for Session {
    type Context = Context<Self>;
}

pub struct Sessions {
    pub sessions: HashMap<usize, Session>,
    pub rng: ThreadRng,
}

impl Sessions {
    pub fn new() -> Self {
        Sessions {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn get(&self, sid: &usize) -> Option<&Session> {
        self.sessions.get(sid)
    }
}

impl Actor for Sessions {
    type Context = Context<Self>;
}

impl Handler<Connect> for Sessions {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let Connect { uid, addr } = msg;
        let sid = self.rng.gen::<usize>();
        self.sessions.insert(sid, Session::new(sid, addr, User {}));
        sid
    }
}

impl Handler<DisConnect> for Sessions {
    type Result = ();

    fn handle(&mut self, msg: DisConnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}
