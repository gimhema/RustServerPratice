pub mod ServerBaseModule;
pub mod ServerFunctions;
pub mod GamePacketModule;
pub mod ConnetionHandleModule;

pub mod Server {
    use super::ServerBaseModule::ServerBase;
    use super::GamePacketModule::GamePacket;
    use super::ConnetionHandleModule::ConnectionHandler;
}
