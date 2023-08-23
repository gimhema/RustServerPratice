// Handling Client's Information

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;
use mio::net::{TcpListener, TcpStream};
use mio::{Token};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::GameLogicCore::GameLogicCore::GamePlayer;
use crate::sendMessageBuffer;
// use crate::someGlobal;
use crate::GameLogicCore;



const SENDTEST: &[u8] = b"Hi Unreal This Message Is Update Loop\n";

pub fn SendTestFunction(connectionStream: &mut TcpStream)
{
    connectionStream.write(SENDTEST);
}

pub fn Initailize() {
    println!("Arena Client Initialize");
}

pub struct ArenaPlayer {
    userID: i64,
    userName: String,
    playerLogic: GamePlayer
}

pub struct ArenaClientNetworkInfo {
    userToken: Token,
    userIdentify: i64
}

impl  ArenaPlayer {
    pub fn Create(&mut self, ID: i64, userName: String, IP: String) {
        self.userID = ID;
        self.userName = userName;
    }
    pub fn Connect(&mut self) {

    }

}


