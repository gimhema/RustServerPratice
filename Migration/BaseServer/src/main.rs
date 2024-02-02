mod Server;
mod GameLogic;

use Server::ServerBaseModule::ServerBase;
use Server::MessageBufferModule::RecvMessageBuffer;
use Server::MessageBufferModule::SendMessageBuffer;
use std::sync::Mutex;
use std::collections::HashMap;
use std::{thread, time};
use std::sync::{RwLock, Arc};
use Server::GamePacketModule::GamePacket;
use Server::ServerFunctions::*;

extern crate lazy_static;
use lazy_static::lazy_static;

type ArenaEventAction = fn(&mut ServerBase, GamePacket) -> FunctionCallResult;

lazy_static! {
    static ref THREAD_SWITCH: Mutex<bool> = Mutex::new(false); 
    static ref gRecvMessageBuffer: RecvMessageBuffer = RecvMessageBuffer::new();
    static ref gSendMessageBuffer: SendMessageBuffer = SendMessageBuffer::new();
    static ref gServer: Arc<RwLock<ServerBase>> = Arc::new(RwLock::new(ServerBase::new()));
    static ref serverActionMap: Mutex<HashMap<i64, ArenaEventAction>> = Mutex::new(HashMap::new());
}

pub fn GetThreadSwitch() -> bool
{
    THREAD_SWITCH.lock().unwrap().clone()
}

pub fn SetThreadSwitch(val: bool)
{
    let mut switch = THREAD_SWITCH.lock().unwrap();
    *switch = val;
}


 pub fn ServerRunThreadWorker()
 {
    {
        let mut server = gServer.write().unwrap();
        server.Start();
    }
 }

fn main() {
    println!("Start Server . . .");
    
    let mut server = gServer.write().unwrap();
    server.Start();
}
