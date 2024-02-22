
use std::borrow::BorrowMut;

use super::{GamePacketModule::GamePacket, ServerBaseModule::ServerBase};
use crate::{gServer, serverActionMap};
use super::ServerFuncitonMap::*;

// Server Functions . . .
// success
pub enum FunctionCallResult {
    FUNCTION_CALL_SUCCESS,
    FUNCTION_CALL_FAIL,
}

pub enum FunctionHeader {
    DEFAULT,
    CHAT_MESSAGE_ALL,
    CHAT_MESSAGE_TO_ONE,    
    CHAT_MESSAGE_TO_GROUP,
    MOVE_TO_LOCATION,
    CONNECTION_SUCESSFUL,
} 

impl FunctionHeader {
    fn ServerActionByFunctionHeader(&self, val : GamePacket)
    {
        match self {
            FunctionHeader::DEFAULT => {

            }
            FunctionHeader::CHAT_MESSAGE_ALL => {
                ServerAction_CHAT_MESSAGE_ALL(val);
            }
            FunctionHeader::CHAT_MESSAGE_TO_GROUP => {
                ServerAction_CHAT_MESSAGE_TO_GROUP(val);
            }
            FunctionHeader::CHAT_MESSAGE_TO_ONE => {
                ServerAction_CHAT_MESSAGE_TO_ONE(val);
            }
            FunctionHeader::MOVE_TO_LOCATION => {
                ServerAction_MOVE_TO_LOCATION(val);
            }
            FunctionHeader::CONNECTION_SUCESSFUL => {

            }
        }
    }
}

pub fn CallServerActionByFunctionHeader(msg : Option<String>) {

    let mut _packet = GamePacket::GamePacketDeSerialize(msg.as_ref().unwrap().as_str());
    let _funcID = _packet.as_ref().unwrap().getFunctionHeader().clone();
    let _funcHeader : FunctionHeader = _funcID.into();
    _funcHeader.ServerActionByFunctionHeader(_packet.unwrap());
}





