mod Server;
use Server::ServerBaseModule::ServerBase;
use Server::MessageBufferModule::RecvMessageBuffer;
use Server::MessageBufferModule::SendMessageBuffer;
use std::sync::Mutex;
// use Server::GamePacketModule::GamePacket;
// use Server::GamePacketModule::PacketTestManager;

extern crate lazy_static;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_SWITCH: Mutex<bool> = Mutex::new(false);
    static ref gRecvMessageBuffer: RecvMessageBuffer = RecvMessageBuffer::new();
    static ref gSendMessageBuffer: SendMessageBuffer = SendMessageBuffer::new();
}

fn main() {
    println!("Start Server . . .");

//    let server = ServerBase();

//    let mut packetTest = PacketTestManager{};
//
//    packetTest.PacketTest();

    let mut server = ServerBase::new();
    server.Start();


}
