mod models;
mod utils;
mod errors;
mod websocket;


use futures::StreamExt;
use redis::aio::PubSub;
use tokio::{net::TcpListener, task};

use anyhow::{Context};

use websocket::{handle_new_conn};

use utils::deserialize_redis_msg;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let redis_client = redis::Client::open("redis://localhost:6379")?;

    let socket = TcpListener::bind("127.0.0.1:1200")
        .await
        .with_context(|| format!("Couldn't start a websocket."))?;

    while let Ok((stream, addr)) = socket.accept().await {
        println!("new connection on IP {}", addr);

        let mut conn = redis_client.get_async_connection().await?;



    
        let mut pubsub = conn.into_pubsub();

        if let Ok(pusub) = pubsub.subscribe("mochi-events").await {
            println!("pubsub connected");

            task::spawn(handle_new_conn(stream, addr, pubsub));
        } else {
            println!("pubsub not connected");
        }
    }

    Ok(())
}
