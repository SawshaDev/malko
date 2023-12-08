use redis::Msg;
use serde::Deserialize;
use tokio::{sync::Mutex, net::TcpStream};
use futures::{stream::{SplitSink}, SinkExt};

use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use std::{error::Error, fmt::Display, sync::Arc};

use crate::{errors::{ServerError, Result}};

use crate::models::{ClientMessage, ClientEvents};

pub async fn send_payload(
    tx: &Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    payload: &ClientEvents,
) {
    if let Err(err) = tx
        .lock()
        .await
        .send(Message::Text(
            serde_json::to_string(payload).unwrap(),
        ))
        .await
    {
        println!("Could not send payload: {}", err);
    }
}

/// An Error that represents a Payload not being found.
#[derive(Debug)]
pub struct PayloadNotFound;

impl Display for PayloadNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Payload Not Found")
    }
}

impl Error for PayloadNotFound {}

/// A function that simplifies deserializing a message Payload.
pub fn deserialize_redis_msg(payload: Msg) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(serde_json::from_str::<serde_json::Value>(
        &payload
            .get_payload::<String>()
            .map_err(|_|PayloadNotFound)?,
    )?.to_string())
}