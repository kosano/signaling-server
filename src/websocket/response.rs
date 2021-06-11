use serde::Serialize;

pub struct Response {}

impl Response {
    pub fn marshal(event: &str, result: &impl Serialize) -> String {
        format!(
            "{{\"{}\": {}}}",
            event,
            serde_json::to_string_pretty(result).unwrap()
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CreateRoomResponse {
    pub id: String,
    pub name: Option<String>,
}

impl CreateRoomResponse {
    pub fn new(input: (String, Option<String>)) -> Self {
        CreateRoomResponse {
            id: input.0,
            name: input.1,
        }
    }
    pub fn marshal(&self) -> String {
        Response::marshal("on_create_room", self)
    }
}

#[derive(Debug, Serialize)]
pub struct RoomMessage {
    pub room_id: String,
    pub type_name: String,
    pub data: String,
}

impl RoomMessage {
    pub fn new(input: (String, String, String)) -> Self {
        RoomMessage {
            room_id: input.0,
            type_name: input.1,
            data: input.2,
        }
    }
    pub fn marshal(&self) -> String {
        Response::marshal("on_room_message", self)
    }
}
