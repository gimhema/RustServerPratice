mod Server;
use Server::ServerBaseModule::ServerBase;
use Server::GamePacketModule::GamePacket;
use Server::GamePacketModule::PacketTestManager;

fn main() {
//    println!("Hello, world!");

//    let server = ServerBase();

    let mut packetTest = PacketTestManager{};

    packetTest.PacketTest();
}
