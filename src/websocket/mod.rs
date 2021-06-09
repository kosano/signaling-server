// use crate::room::Room;
use crate::room::status::{Connect, CreateRoom, DisConnect, JoinRoom, Message, SendData};
use crate::room::RoomServer;
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
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        // ctx.run_interval(dur, f);
        ctx.ping("{ping: {}}".as_bytes());
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
                id: self.id,
                addr: addr.recipient(),
                user: self.user,
            })
            .into_actor(self)
            .then(|res, act, _ctx| {
                match res {
                    Ok(res) => act.id = res,
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
        println!("{:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let m = json::parse(text.as_str()).unwrap();
                if m.has_key("create_room") {
                    self.room_server
                        .send(CreateRoom {
                            name: Some("main".to_string()),
                            sid: self.id,
                        })
                        .into_actor(self)
                        .then(|res, _act, ctx| {
                            match res {
                                Ok(id) => {
                                    println!("{}", &id);
                                    ctx.text(format!("{}", id.to_string()))
                                }
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
                            rid: m["join_room"]["room_id"]
                                .to_string()
                                .parse::<usize>()
                                .unwrap()
                                .clone(),
                            sid: self.id,
                        })
                        .into_actor(self)
                        .then(|res, _act, ctx| {
                            match res {
                                Ok(id) => {
                                    println!("{}", &id);
                                    ctx.text(format!("{:?}", id))
                                }
                                // something is wrong with chat server
                                _ => (),
                            }
                            fut::ready(())
                        })
                        .wait(ctx);
                }

                if m.has_key("room_send_data") {
                    match m["room_send_data"]["type"].as_str().clone() {
                        Some("login") => {
                            ctx.text(m["room_send_data"].to_string());
                            return;
                        }
                        _ => {
                            self.room_server
                                .send(SendData {
                                    rid: m["room_send_data"]["room_id"]
                                        .to_string()
                                        .parse::<usize>()
                                        .unwrap(),
                                    msg: m["room_send_data"].to_string(),
                                    sid: self.id,
                                })
                                .into_actor(self);
                        }
                    }
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
