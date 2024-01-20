
use super::ServerBaseModule::ServerBase;
use crate::serverActionMap;
use super::ServerFuncitonMap::*;

// Server Functions . . .

pub struct FunctionParam {
    functionParam: Vec<f64>,
    functionStrParam: String,
}

impl FunctionParam {
    pub fn new (_param: Vec<f64>, _strParam: String) -> Self {
        FunctionParam { functionParam: _param, functionStrParam: _strParam }
    }
}


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
        let funcID = _packet.as_ref().unwrap().getFunctionHeader();
//        let funcParam = _packet.as_ref().unwrap().getFunctionParam();
        let _fParamVec = _packet.as_ref().unwrap().getFunctionParam();
        let _fStrParam = _packet.as_ref().unwrap().getFunctionStrParam();
        let _functionParam = FunctionParam::new(_fParamVec.clone(), _fStrParam.clone());

        if let Some(server_action) = server_action_map.get(funcID) {
        let result = server_action(_functionParam);
        println!("Result: {}", result);
        } else {
            println!("Function not found");
        }
        
    }

    // Write in Function Map

}
