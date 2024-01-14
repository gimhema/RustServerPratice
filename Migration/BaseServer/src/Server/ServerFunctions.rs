
use super::ServerBaseModule::ServerBase;


// Server Functions . . .

pub enum FunctionHeader {
    CHAT_MESSAGE_ALL,
    CHAT_MESSAGE_TO_ONE,    
    CHAT_MESSAGE_TO_GROUP,
} 

impl ServerBase {
    pub fn CallRecvFunctionByMessage(&mut self, msg : Option<String>)
    {
        println!("Received Message : {}", msg.unwrap().as_str());
    }
}


