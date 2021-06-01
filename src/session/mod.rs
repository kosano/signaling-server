use crate::user::User;
use actix::prelude::{Actor, Context, Handler, Recipient, SendError};
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::HashMap;
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

// impl actix::Message for Message {
//     type Result = ();
// }

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

pub struct Session {
    pub id: usize,
    pub addr: Recipient<Message>,
    pub user: User,
}

impl Session {
    pub fn new(addr: Recipient<Message>, user: User) -> Self {
        Session {
            id: rand::thread_rng().gen::<usize>(),
            addr: addr,
            user: user,
        }
    }
    // pub fn reset(&mut self, addr: Recipient<Message>, user: User) -> &Self {
    //     self.id = rand::thread_rng().gen::<usize>();
    //     self.addr = Some(addr);
    //     self.user = user;
    //     self
    // }

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
}

impl Actor for Sessions {
    type Context = Context<Self>;
}

// impl Handler<Connect> for Session {
//     type Result = usize;

//     fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
//         let Connect { uid, addr } = msg;
//         let sid = self.rng.gen::<usize>();
//         self.sessions.insert(
//             sid,
//             Session {
//                 id: sid,
//                 addr: addr,
//                 user: User {},
//             },
//         );
//         uid
//     }
// }

impl Handler<DisConnect> for Sessions {
    type Result = ();

    fn handle(&mut self, msg: DisConnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}
