
use super::ServerBaseModule::ServerBase;
use super::GamePacketModule::GamePacket;
use super::ServerFunctions::*;
use super::MessageBufferModule::*;
use super::GameLogic::GameLogicBaseModule::*;
use crate::{gSendMessageBuffer, gGameLogic};


// need ServerBaser -> GameLogic
pub fn ServerAction_CHAT_MESSAGE_ALL(server: &mut ServerBase, val : GamePacket) -> FunctionCallResult {
    // println!("{}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;
    let mut _numUser = server.GetNumUser().clone();

    if (_numUser <= 0) {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }
    else {
        result = FunctionCallResult::FUNCTION_CALL_SUCCESS;        
    }

    let mut _sendID = val.getSenderID();
    let mut _fParamVec = val.getFunctionParam();
    let mut _fParamStr = val.getFunctionStrParam();
    let mut _fHeader = val.getFunctionHeader();

    for i in 0.._numUser {
        // 벡터의 각 요소 출력
        let mut _packet = GamePacket::new(i, *_sendID,
            *_fHeader, _fParamVec.clone(), _fParamStr.clone() );

        gSendMessageBuffer.PushBackData(_packet);
    }

    result
}

pub fn ServerAction_CHAT_MESSAGE_TO_ONE(server: &mut ServerBase, val : GamePacket) -> FunctionCallResult {
    // println!("Message Test 2 {}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;
    let mut _numUser = server.GetNumUser().clone();

    if (_numUser <= 0) {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }
    else {
        result = FunctionCallResult::FUNCTION_CALL_SUCCESS;        
    }

    let mut _sendID = val.getSenderID();
    let mut _targetID = val.getTargetID();
    let mut _fParamVec = val.getFunctionParam();
    let mut _fParamStr = val.getFunctionStrParam();
    let mut _fHeader = val.getFunctionHeader();

    let mut _packet = GamePacket::new(*_targetID, *_sendID,
        *_fHeader, _fParamVec.clone(), _fParamStr.clone() );

    gSendMessageBuffer.PushBackData(_packet);

    result
}

pub fn ServerAction_CHAT_MESSAGE_TO_GROUP(server: &mut ServerBase, val : GamePacket) -> FunctionCallResult {
    // println!("Message Test 3 {}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;
    let mut _numUser = server.GetNumUser().clone();

    if (_numUser <= 0) {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }
    else {
        result = FunctionCallResult::FUNCTION_CALL_SUCCESS;        
    }


    result
}