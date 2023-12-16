
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use super::ServerBaseModule::ServerBase;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GamePacket {
    senderID: i64,
    functionHeader: i64,
    functionParam: Vec<f64>
}

impl ServerBase {

}
