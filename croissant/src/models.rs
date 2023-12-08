use serde::{Deserialize, Serialize};


/// The message payload
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub author: String,
    pub content: String,
}

/// The Pandemonium Payload Enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "op", content = "d")]
pub enum APIPayload {
    MessageCreate(Message),
}
