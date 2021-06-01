use actix;

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
