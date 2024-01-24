
use super::{GamePacketModule::GamePacket, ServerBaseModule::ServerBase};
use crate::serverActionMap;
use super::ServerFuncitonMap::*;

// Server Functions . . .

pub enum FunctionHeader {
    CHAT_MESSAGE_ALL,
    CHAT_MESSAGE_TO_ONE,    
    CHAT_MESSAGE_TO_GROUP,
} 

impl ServerBase {
    pub fn InitailizeFunctionMap(&mut self)
    {
        println!("Function Map Initialize");

        serverActionMap.lock().unwrap().insert(
            FunctionHeader::CHAT_MESSAGE_ALL as i64,
            ServerAction_CHAT_MESSAGE_ALL);

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
        println!("Result: {}", result);
        } else {
            println!("Function not found");
        }
        
    }

    // Write in Function Map

}

// pub fn CallServerAction(&mut self, userID : &i64, funcID : &i64, funcParam : &String ) {

//     let server_action_map = &*serverActionMap.lock().unwrap();

//     // call server_action_map[funcID](funcParam) 
//     if let Some(server_action) = server_action_map.get(funcID) {
//     let result = server_action(funcParam.clone());

//     println!("Result: {}", result);
//     } else {
//         println!("Function not found");
//     }
// }

