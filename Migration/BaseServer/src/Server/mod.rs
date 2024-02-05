pub mod ServerBaseModule;
pub mod ServerFunctions;
pub mod GamePacketModule;
pub mod ConnetionHandleModule;
pub mod MessageBufferModule;
pub mod ServerFuncitonMap;
pub mod ServerFunctionHeaderConvert;

use crate::{GameLogic};
use crate::{gRecvMessageBuffer, gSendMessageBuffer};

pub mod Server {
    use super::ServerBaseModule::ServerBase;
    use super::GamePacketModule::GamePacket;
    use super::ConnetionHandleModule::ConnectionHandler;
    use super::MessageBufferModule::RecvMessageBuffer;
    use super::MessageBufferModule::SendMessageBuffer;
    
}
