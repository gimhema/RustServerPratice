
use super::ServerBaseModule::ServerBase;
use super::GamePacketModule::GamePacket;
use crate::{gServer};

pub fn ServerAction_CHAT_MESSAGE_ALL(server: &mut ServerBase, val : GamePacket) -> i64 {
    // println!("{}",  val);
    0
}

pub fn ServerAction_CHAT_MESSAGE_TO_ONE(server: &mut ServerBase, val : GamePacket) -> i64 {
    // println!("Message Test 2 {}",  val);
    0
}

pub fn ServerAction_CHAT_MESSAGE_TO_GROUP(server: &mut ServerBase, val : GamePacket) -> i64 {
    // println!("Message Test 3 {}",  val);
    0
}