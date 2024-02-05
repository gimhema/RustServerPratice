
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
    CHAT_MESSAGE_ALL,
    CHAT_MESSAGE_TO_ONE,    
    CHAT_MESSAGE_TO_GROUP,
} 

pub fn GCallRecvFunctionByMessage(msg : Option<String>)
    {
        let server_action_map = &*serverActionMap.lock().unwrap();

        // let mut _packet = self.GamePacketDeSerialize(msg.as_ref().unwrap().as_str());
        
        let mut _packet = GamePacket::GamePacketDeSerialize(msg.as_ref().unwrap().as_str());

        let pid = _packet.as_ref().unwrap().getSenderID().clone();
        let targetID = _packet.as_ref().unwrap().getTargetID().clone();
        let funcID = _packet.as_ref().unwrap().getFunctionHeader().clone();
        let funcIDref = _packet.as_ref().unwrap().getFunctionHeader();
        let funcParamVec = _packet.as_ref().unwrap().getFunctionParam().clone();
        let funcStr = _packet.as_ref().unwrap().getFunctionStrParam().clone();

        let actionParam = GamePacket::new(pid, targetID, funcID, funcParamVec, funcStr);

        if let Some(server_action) = server_action_map.get(funcIDref) {
        
        let mut _server = gServer.write().unwrap();
        
        let result = server_action(_server.borrow_mut(), actionParam);
        // println!("Result: {}", result);
        } else {
            println!("Function not found");
        }
        
    }

impl ServerBase {
    pub fn InitailizeFunctionMap(&mut self)
    {
        println!("Function Map Initialize");

        serverActionMap.lock().unwrap().insert(
            FunctionHeader::CHAT_MESSAGE_ALL as i64,
            ServerAction_CHAT_MESSAGE_ALL);

        serverActionMap.lock().unwrap().insert(
            FunctionHeader::CHAT_MESSAGE_TO_ONE as i64,
            ServerAction_CHAT_MESSAGE_TO_ONE);

        serverActionMap.lock().unwrap().insert(
            FunctionHeader::CHAT_MESSAGE_TO_GROUP as i64,
            ServerAction_CHAT_MESSAGE_TO_GROUP);            

    }

    pub fn CallRecvFunctionByMessage(&mut self, msg : Option<String>)
    {
        let server_action_map = &*serverActionMap.lock().unwrap();

        let mut _packet = self.GamePacketDeSerialize(msg.as_ref().unwrap().as_str());
        
        let pid = _packet.as_ref().unwrap().getSenderID().clone();
        let targetID = _packet.as_ref().unwrap().getTargetID().clone();
        let funcID = _packet.as_ref().unwrap().getFunctionHeader().clone();
        let funcIDref = _packet.as_ref().unwrap().getFunctionHeader();
        let funcParamVec = _packet.as_ref().unwrap().getFunctionParam().clone();
        let funcStr = _packet.as_ref().unwrap().getFunctionStrParam().clone();

        let actionParam = GamePacket::new(pid, targetID, funcID, funcParamVec, funcStr);

        if let Some(server_action) = server_action_map.get(funcIDref) {
        let result = server_action(self, actionParam);
        // println!("Result: {}", result);
        } else {
            println!("Function not found");
        }
        
    }

}


