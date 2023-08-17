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

pub struct ConnectInfo {
    connectToken: Token,
    connectStream: TcpStream
}

impl ConnectInfo {
    pub fn Create(&mut self, token: Token, stream: TcpStream) 
    {
        self.connectToken = token;
    }
}


pub struct ArenaPlayer {
    userID: i64,
    userName: String,
    playerLogic: GamePlayer
}

pub struct ArenaClientNetworkInfo {
    userToken: Token,
    userConnectStream: TcpStream,
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

pub struct ArenaClientManager {
    clientContainer: HashMap<i64, ArenaPlayer>, // ID, Client Information
    clientNetworkContainer: HashMap<Token, ArenaClientNetworkInfo>, // ID, Arena Network Information
    clientTokenVec: Vec<Token>
}

impl ArenaClientManager {
    pub fn Initialize(&mut self)
    {
        println!("Arena Client Manager Initialize");
        
    }

    pub fn new() -> ArenaClientManager {
        ArenaClientManager { 
            clientContainer: HashMap::new(),
            clientNetworkContainer: HashMap::new(),
            clientTokenVec: Vec::new()
        }
    }

    pub fn testMethod() {
//        someGlobal = 3;
//        sendMessageBuffer.lock().unwrap().push_back("test");
        sendMessageBuffer.lock().unwrap().push_back("value");
    }

}

