use std::borrow::Cow;
use std::net::{SocketAddr};

use futures::stream::{SplitSink, SplitStream};
use futures::{StreamExt, SinkExt};

use std::sync::Arc;
use redis::aio::PubSub;
use tokio::sync::Mutex;
use tokio::{
    net::{TcpStream},
    task,
};

use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::{Message};

use crate::utils::{send_payload, deserialize_redis_msg};

use crate::models::{ClientEvents, ClientMessage};


pub async fn handle_new_conn(stream: TcpStream, _addr: SocketAddr, pubsub: PubSub){
    if let Ok(ws) = accept_async(stream).await {
        let (write, mut read) = ws.split();

        let write = Arc::new(Mutex::new(write));


        send_payload(&write, &ClientEvents::Authenticated).await;

        let handle_read = async {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(data) => match data {
                        Message::Text(message) => {
                            match serde_json::from_str::<ClientMessage>(&message) {
                                Ok(ClientMessage::Ping) => {
                                    send_payload(&write, &ClientEvents::Pong {data: "pp".to_string()}).await;
                                }
                                _ => println!("Unknown gateway payload: {}", message),
                            }
                        }
                        _ =>  println!("Unsupported Gateway message type.\n{:#?}", data),


                    }
                    Err(_) => break,
                }
            }
        };

        let handle_redis = async {
            pubsub
                .into_on_message()
                .for_each(|msg| async move {
                    println!("{:#?}", msg.get_payload::<String>().unwrap())
                }).await


        };

        tokio::select! {
            _ = handle_read => {},
            _ = handle_redis => {}
        }

    }


}

async fn close_socket(
    tx: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    rx: SplitStream<WebSocketStream<TcpStream>>,
    frame: CloseFrame<'_>,
) {
    let tx = Arc::try_unwrap(tx).expect("Couldn't obtain tx from MutexLock");
    let tx = tx.into_inner();

    if let Err(err) = tx
        .reunite(rx)
        .expect("Couldn't reunite WebSocket stream")
        .close(Some(frame))
        .await
    {
        println!("Couldn't close socket with {}", err);
    }
}