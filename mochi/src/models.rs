use serde::{Deserialize, Serialize};




#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum ClientMessage {
    Authenticate { token: String },
    Ping,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ClientEvents {
    Authenticated,

    Pong {data: String},
}