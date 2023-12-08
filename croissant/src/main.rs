mod models;

#[macro_use]
extern crate rocket;

use anyhow  ;

use rocket::{Rocket, Build, Config};
use rocket::serde::json::{Json, serde_json};


use rocket_db_pools::{Database, deadpool_redis, Connection};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use tokio::io;

use models::{Message, APIPayload};

#[derive(Database)]
#[database("cache")]
pub struct Cache(deadpool_redis::Pool);

#[post("/message", data = "<message>")]
async fn index(
    message: Json<Message>,
    mut cache: Connection<Cache>
) -> io::Result<Json<Message>>{

    let message = message.into_inner();


    let payload = APIPayload::MessageCreate(message);
    cache
        .publish::<&str, String, ()>("mochi-events", serde_json::to_string(&payload).unwrap())
        .await
        .unwrap();

    // rust sees this as a weird decision, but it'll make more sense once more things are added to the message create.
    if let APIPayload::MessageCreate(message) = payload {
        Ok(Json(message))
    } else {
        unreachable!()
    }
}

pub fn rocket() -> Result<Rocket<Build>, anyhow::Error> {
    let config = Config::figment()
        .merge(("port",
            9934
        ))
        .merge(("databases.cache",
            rocket_db_pools::Config {
            url: "redis://127.0.0.1:6379".to_string(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
        },));

    Ok(rocket::custom(config)
        .mount("/", routes![index])
        .attach(Cache::init())
    )
}   

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    let _ = rocket()?
        .launch()
        .await;

    Ok(())
}