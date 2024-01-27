
use super::ServerBaseModule::ServerBase;
use super::GamePacketModule::GamePacket;
use super::ServerFunctions::*;

pub fn ServerAction_CHAT_MESSAGE_ALL(server: &mut ServerBase, val : GamePacket) -> FunctionCallResult {
    // println!("{}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;
    let mut _numUser = server.GetNumUser().clone();

    if (_numUser <= 0)
    {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }

    result
}

pub fn ServerAction_CHAT_MESSAGE_TO_ONE(server: &mut ServerBase, val : GamePacket) -> i64 {
    // println!("Message Test 2 {}",  val);
    0
}

pub fn ServerAction_CHAT_MESSAGE_TO_GROUP(server: &mut ServerBase, val : GamePacket) -> i64 {
    // println!("Message Test 3 {}",  val);
    0
}