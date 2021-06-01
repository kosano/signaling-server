use crate::user::User;
use actix::prelude::{Actor, Context, Handler, Recipient, SendError};
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::HashMap;
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(actix::Message)]
#[rtype(usize)]
pub struct Connect {
    pub session: Session,
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

#[derive(Clone)]
pub struct Session {
    pub id: usize,
    pub addr: Recipient<Message>,
    pub user: User,
}

#[derive(Clone)]
pub struct Room {
    pub id: usize,
    pub name: Option<String>,
    pub sessions: HashMap<usize, Session>,
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Room {
    pub fn new() -> Self {
        Room {
            id: 0,
            name: None,
            sessions: HashMap::new(),
        }
    }

    pub fn join(&mut self, session: Session) -> &Self {
        self.room_send(
            format!("User {:?} Join Room.", session.id).as_str(),
            &session.id,
        );
        self.sessions.insert(session.id, session);
        self
    }

    pub fn leave(&mut self, sid: &usize) -> &Self {
        self.room_send(format!("User {:?} Leave Room.", *sid).as_str(), sid);
        match self.sessions.get(sid) {
            Some(_) => self.sessions.remove(sid),
            _ => None,
        };
        self
    }

    fn room_send(&self, msg: &str, skip_id: &usize) {
        for (id, session) in self.sessions.iter() {
            if *id != *skip_id {
                let _ = session.addr.do_send(Message(msg.to_owned()));
            }
        }
    }
}

impl Actor for RoomServer {
    type Context = Context<Self>;
}

pub struct RoomServer {
    pub sessions: HashMap<usize, Session>,
    pub rooms: HashMap<usize, Room>,
    pub rng: ThreadRng,
}

impl RoomServer {
    pub fn new() -> Self {
        RoomServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn create_room(&mut self, name: Option<String>, session: Session) -> &Room {
        let rid = self.rng.gen::<usize>();
        let mut room = Room::new();
        room.id = rid;
        room.name = name;
        room.join(session);
        self.rooms.insert(rid, room.clone());
        self.rooms.get(&rid).unwrap()
    }

    pub fn join_room(&mut self, rid: &usize, sid: &usize) -> &Room {
        for (_, room) in self.rooms.iter_mut() {
            room.leave(sid);
        }
        self.rooms
            .get_mut(rid)
            .unwrap()
            .join(self.sessions.get(sid).unwrap().clone())
        // self.send_message(rid, format!("User: {}, join Room.", sid).as_str(), sid);
        // self.rooms.get(rid).unwrap()
    }

    pub fn send_message(&self, rid: &usize, msg: &str, skip_id: &usize) {
        self.rooms.get(rid).unwrap().room_send(msg, skip_id)
    }
}

impl Handler<Connect> for RoomServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("User {:?} Connect.", msg.session.id);
        self.sessions.insert(msg.session.id, msg.session.clone());
        msg.session.id
    }
}

impl Handler<DisConnect> for RoomServer {
    type Result = ();

    fn handle(&mut self, msg: DisConnect, _: &mut Context<Self>) -> Self::Result {
        println!("User {:?} DisConnect.", msg.id);
        for (_, room) in self.rooms.iter_mut() {
            room.leave(&msg.id);
        }
        self.sessions.remove(&msg.id);
    }
}

impl Handler<CreateRoom> for RoomServer {
    type Result = usize;

    fn handle(&mut self, msg: CreateRoom, _: &mut Context<Self>) -> Self::Result {
        println!("User {:?} CreateRoom.", msg.sid);
        let room = self.create_room(msg.name, self.sessions.get(&msg.sid).unwrap().clone());
        room.id
    }
}

impl Handler<JoinRoom> for RoomServer {
    type Result = usize;

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) -> Self::Result {
        let room = self.join_room(&msg.rid, &msg.sid);
        room.id
    }
}

impl Handler<SendData> for RoomServer {
    type Result = ();

    fn handle(&mut self, msg: SendData, _: &mut Context<Self>) {
        self.send_message(&msg.rid, msg.msg.as_str(), &msg.sid);
    }
}
