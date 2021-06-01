use crate::room::RoomServer;
use crate::session::Sessions;
use crate::websocket;
use actix::Actor;
use actix_web::{web, App, HttpServer};

pub struct ServerConfig {
    pub listen: String,
    pub workers: usize,
}

impl ServerConfig {
    pub fn new(listen: String, workers: usize) -> Self {
        ServerConfig {
            listen: listen,
            workers: workers,
        }
    }
}

pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: (&str, usize)) -> Self {
        Server {
            config: ServerConfig::new(config.0.to_string(), config.1),
        }
    }

    pub async fn listen(&self) -> std::io::Result<()> {
        let room_server = RoomServer::new().start();
        HttpServer::new(move || {
            App::new()
                .data(room_server.clone())
                .route("/ws/{token}", web::get().to(websocket::ws_handler))
        })
        .bind(&self.config.listen.to_string())?
        .workers(self.config.workers)
        .run()
        .await
    }
}
