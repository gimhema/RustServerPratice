
use super::ServerBaseModule::ServerBase;
use crate::serverActionMap;
use super::ServerFuncitonMap;

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

//        serverActionMap.lock().unwrap().insert(
//            MessageUnique::CONSOLE_WRITE_LINE as i64,
//             ServerAction_ConsoleWriteLineTest);

    }

    pub fn CallRecvFunctionByMessage(&mut self, msg : Option<String>)
    {
        println!("Received Message : {}", msg.unwrap().as_str());
    }

    // Write in Function Map

}


