
use std::result;

use super::GamePacketModule::SendGamePacket;
use super::ServerBaseModule::ServerBase;
use super::GamePacketModule::GamePacket;
use super::ServerFunctions::*;
use super::MessageBufferModule::*;
use super::GameLogic::GameLogicBaseModule::*;
use crate::GameCommon::Math::FLocation;
use crate::{gSendMessageBuffer, gGameLogic};

pub fn ServerAction_CONNECTION_SUCESSFUL(val : GamePacket) -> FunctionCallResult {
    let mut result = FunctionCallResult::FUNCTION_CALL_SUCCESS;

    

    println!("Call Func ServerAction_CONNECTION_SUCESSFUL : Connect Successful ! ! !");

    let mut _gameLogic = gGameLogic.write().unwrap();
    let mut _numUser = _gameLogic.GetUserNum().clone();
    
    for i in 0.._numUser {
        // 벡터의 각 요소 출력
        let mut _packet = GamePacket::new(i, 0,
            FunctionHeader::CONNECTION_SUCESSFUL as i64,
            vec![0.0],
              "Hi Unreal Clients ! !".to_string() );
        // gSendMessageBuffer.PushBackData(_packet);
        SendGamePacket(Some(_packet)); // 푸시하지말고 바로 보낸다.
    }

    result
}


// need ServerBaser -> GameLogic
pub fn ServerAction_CHAT_MESSAGE_ALL(val : GamePacket) -> FunctionCallResult {
    // println!("{}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;

    let mut _gameLogic = gGameLogic.write().unwrap();
    let mut _numUser = _gameLogic.GetUserNum().clone();

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
        // gSendMessageBuffer.PushBackData(_packet);
        SendGamePacket(Some(_packet)); // 푸시하지말고 바로 보낸다.
    }

    result
}

pub fn ServerAction_CHAT_MESSAGE_TO_ONE(val : GamePacket) -> FunctionCallResult {
    // println!("Message Test 2 {}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;
    let mut _gameLogic = gGameLogic.write().unwrap();
    let mut _numUser = _gameLogic.GetUserNum().clone();

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

pub fn ServerAction_CHAT_MESSAGE_TO_GROUP(val : GamePacket) -> FunctionCallResult {
    // println!("Message Test 3 {}",  val);
    let mut result = FunctionCallResult::FUNCTION_CALL_FAIL;
    let mut _gameLogic = gGameLogic.write().unwrap();
    let mut _numUser = _gameLogic.GetUserNum().clone();

    if (_numUser <= 0) {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }
    else {
        result = FunctionCallResult::FUNCTION_CALL_SUCCESS;        
    }

    result
}

pub fn ServerAction_MOVE_TO_LOCATION(val : GamePacket) -> FunctionCallResult {
    let mut result = FunctionCallResult::FUNCTION_CALL_SUCCESS;

    let mut _gameLogic = gGameLogic.write().unwrap();
    let mut _numUser = _gameLogic.GetUserNum().clone();

    if (_numUser <= 0) {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }
    else {
        // 이동할 플레이어
        let _moveCharacterId = val.getSenderID();
        let _moveLocX = val.getFunctionParam()[0];
        let _moveLocY = val.getFunctionParam()[1];
        let _moveLocZ = val.getFunctionParam()[2];

        // 캐릭터 위치 업데이트
        
        let _new_location = FLocation::MakeLocationByVec(vec![_moveLocX, _moveLocY, _moveLocZ]);

        let mut _pID = *val.getSenderID() as usize; 
        // let mut _character = _gameLogic.GetCharacterManager().GetCharacterByID(_pID);
        // _character.SetWorldLocation(_new_location);
        let _cm = _gameLogic.GetCharacterManager();
        _cm.SetWorldLocaitonByPID(_pID, _new_location);

        let _fHeader : i64 = FunctionHeader::MOVE_TO_LOCATION.into();
        let _fParamVec = vec![_moveLocX, _moveLocY, _moveLocZ];
        let _fParamStr = "".to_string();
    
        for i in 0.._numUser {
            // 벡터의 각 요소 출력
            let mut _packet = GamePacket::new(i, *_moveCharacterId,
                _fHeader, _fParamVec.clone(), _fParamStr.clone() );
            // gSendMessageBuffer.PushBackData(_packet);
            SendGamePacket(Some(_packet)); // 푸시하지말고 바로 보낸다.
        }    
    }

    
    result
}

pub fn ServerAction_INIT_TRANSFORM(val : GamePacket) -> FunctionCallResult {
    let mut result = FunctionCallResult::FUNCTION_CALL_SUCCESS;

    // 다시 응답하지는않고 초기의 캐릭터 트랜스폼 정보를 초기화함

    let mut _gameLogic = gGameLogic.write().unwrap();
    let mut _numUser = _gameLogic.GetUserNum().clone();

    if (_numUser <= 0) {
        result = FunctionCallResult::FUNCTION_CALL_FAIL;
    }
    else {
        
        let _moveCharacterId = val.getSenderID();
        let locX = val.getFunctionParam()[0];
        let locY = val.getFunctionParam()[1];
        let locZ = val.getFunctionParam()[2];
        let roll = val.getFunctionParam()[3];
        let pitch = val.getFunctionParam()[4];
        let yaw = val.getFunctionParam()[5];
        


    }



    result
}