// use crate::room::Room;
use crate::room::{
    Connect, CreateRoom, DisConnect, JoinRoom, Message, RoomServer, SendData, Session,
};
use crate::user::User;
use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::{self, Rng};
use std::time::{Duration, Instant};
extern crate json;
/// Define HTTP actor
pub struct WsSession {
    pub id: usize, // session id
    pub hb: Instant,
    pub user: User,             // user info and conf
    pub current_room_id: usize, // join room
    pub room_server: Addr<RoomServer>,
}

impl WsSession {
    pub fn new(user: User, room_server: Addr<RoomServer>) -> Self {
        WsSession {
            id: rand::thread_rng().gen::<usize>(),
            hb: Instant::now(),
            user: user,
            current_room_id: 0,
            room_server: room_server,
        }
    }
    // heath check
    fn hb(&self, _ctx: &mut ws::WebsocketContext<Self>) {
        // ctx.run_interval(dur, f);
    }
}

impl Handler<Message> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.room_server
            .send(Connect {
                session: Session {
                    id: self.id,
                    addr: addr.recipient(),
                    user: self.user,
                },
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => (),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.room_server.do_send(DisConnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let m = json::parse(text.as_str()).unwrap();
                if m.has_key("create_room") {
                    self.room_server
                        .send(CreateRoom {
                            name: Some("test".to_string()),
                            sid: self.id,
                        })
                        .into_actor(self)
                        .then(|res, _act, ctx| {
                            match res {
                                Ok(id) => ctx.text(format!("{:?}", id)),
                                // something is wrong with chat server
                                _ => (),
                            }
                            fut::ready(())
                        })
                        .wait(ctx);
                }

                if m.has_key("join_room") {
                    self.room_server
                        .send(JoinRoom {
                            rid: m["join_room"]["room_id"].as_usize().unwrap(),
                            sid: self.id,
                        })
                        .into_actor(self)
                        .then(|res, _act, ctx| {
                            match res {
                                Ok(id) => ctx.text(format!("{:?}", id)),
                                // something is wrong with chat server
                                _ => (),
                            }
                            fut::ready(())
                        })
                        .wait(ctx);
                }

                if m.has_key("room_send_data") {
                    self.room_server
                        .send(SendData {
                            rid: m["room_send_data"]["room_id"].as_usize().unwrap(),
                            msg: m["room_send_data"]["message"].to_string(),
                            sid: self.id,
                        })
                        .into_actor(self)
                        .then(|res, _act, ctx| {
                            match res {
                                Ok(id) => ctx.text(format!("{:?}", id)),
                                // something is wrong with chat server
                                _ => (),
                            }
                            fut::ready(())
                        })
                        .wait(ctx);
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => (),
        }
    }
}

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    rsrv: web::Data<Addr<RoomServer>>,
) -> Result<HttpResponse, Error> {
    let res = ws::start(
        WsSession::new(User::new(), rsrv.get_ref().clone()),
        &req,
        stream,
    );
    res
}
