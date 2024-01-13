mod Server;
use Server::ServerBaseModule::ServerBase;
use Server::MessageBufferModule::RecvMessageBuffer;
use Server::MessageBufferModule::SendMessageBuffer;
use std::sync::Mutex;
use std::{thread, time};
use std::sync::{RwLock, Arc};
// use Server::GamePacketModule::GamePacket;
// use Server::GamePacketModule::PacketTestManager;

extern crate lazy_static;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_SWITCH: Mutex<bool> = Mutex::new(false); // 원자성을 띠고있는 값으로 바꾸든가 해야한다..
    static ref gRecvMessageBuffer: RecvMessageBuffer = RecvMessageBuffer::new();
    static ref gSendMessageBuffer: SendMessageBuffer = SendMessageBuffer::new();
//    static ref gServer: Mutex<ServerBase> = Mutex::new(ServerBase::new());
    static ref gServer: Arc<RwLock<ServerBase>> = Arc::new(RwLock::new(ServerBase::new()));
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
    loop {
        if(GetThreadSwitch() == true) {break;} // 10초마다 검사하든지 해야한다.

        thread::sleep(time::Duration::from_millis(1000));
        println!("Receive Loop");

        if (gRecvMessageBuffer.IsEmpty() == false)
        {
            let mut _recvMsg = gRecvMessageBuffer.PopData();

            println!("Received Message : {}", _recvMsg.unwrap().as_str());
        }

        let mut server = gServer.write().unwrap();
        server.CallRecvFunction();
        drop(server);
        // recvMsg 가지고 뭘 한다.... 
    }
 }

 pub fn UpdateThreadWorker()
 {
    loop  {
        if(GetThreadSwitch() == true) {break;} // 10초마다 검사하든지 해야한다.

        let mut server = gServer.write().unwrap();
        server.Update();

        // thread::sleep(time::Duration::from_millis(400));
        
    }
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
    
 //    let mut server = gServer.lock().unwrap();

    // Create Receive Messga Thread

    let serverLogic = thread::spawn(move || {
        // recv logic thread
        ServerRunThreadWorker();
    });


    serverLogic.join().unwrap();
}
