mod Server;
use Server::ServerBaseModule::ServerBase;
use Server::MessageBufferModule::RecvMessageBuffer;
use Server::MessageBufferModule::SendMessageBuffer;
use std::sync::Mutex;
use std::{thread, time};
// use Server::GamePacketModule::GamePacket;
// use Server::GamePacketModule::PacketTestManager;

extern crate lazy_static;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_SWITCH: Mutex<bool> = Mutex::new(false);
    static ref gRecvMessageBuffer: RecvMessageBuffer = RecvMessageBuffer::new();
    static ref gSendMessageBuffer: SendMessageBuffer = SendMessageBuffer::new();
    static ref gServer: Mutex<ServerBase> = Mutex::new(ServerBase::new());
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

pub fn RecvThreadWorker()
{
    while (true) {
        if(GetThreadSwitch() == true) {break;}

        let mut _recvMsg = gRecvMessageBuffer.PopData();

        println!("Received Message : {}", _recvMsg.unwrap().as_str());
        // recvMsg 가지고 뭘 한다.... 
    }
 }

fn main() {
    println!("Start Server . . .");
    
    let mut server = gServer.lock().unwrap();

    // Create Receive Messga Thread
    let recvMsgLogic = thread::spawn(move || {
        // recv logic thread
        RecvThreadWorker();
    });

    server.Start();

    recvMsgLogic.join().unwrap();
}
