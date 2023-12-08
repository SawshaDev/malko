use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerError {
    InternalError
}


pub type Result<T, E = ServerError> = std::result::Result<T, E>;