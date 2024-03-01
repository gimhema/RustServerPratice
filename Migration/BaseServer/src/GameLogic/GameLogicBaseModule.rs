
use std::{thread, time};
use std::time::Duration;
use crate::GameCommon::Manager::Manager;
use crate::{gSendMessageBuffer, gRecvMessageBuffer, gServer};
use crate::{GetThreadSwitch, SetThreadSwitch};
use crate::Server::GamePacketModule::*;
use crate::Server::ServerFunctions::*;
use crate::GameLogic::CharacterModule::*;

const GAME_LOGIC_UPDATE_TICK: u64 = 3000;
const MAX_NUM_USER: i64 = 2;

pub struct GameLogicBase {
    updateCount : i64,
    userNum : i64,
    characterManager : CharacterManager
}

impl GameLogicBase {

    pub fn new() -> Self {
        let mut _characterManager = CharacterManager::new();
        GameLogicBase { updateCount : 0, userNum : 0, characterManager : _characterManager }
    }

    pub fn GetCharacterManager(&mut self) -> &mut CharacterManager {
        &mut self.characterManager
    }    

    pub fn GetUserNum(&mut self) -> &i64 {
        &self.userNum
    }

    pub fn IncreaseUserNum(&mut self) {
        let mut _temp = self.userNum.clone() + 1;
        if(_temp >= MAX_NUM_USER) {
            self.userNum = MAX_NUM_USER;
        }
        else {
            self.userNum = _temp;
        }
    }

    pub fn AddNewPlayer(&mut self, pid : i64)
    {
        println!("Add New Player 2 . . ");
        let mut _newPlayer = Character::new();
        _newPlayer.SetPID(pid);
        self.characterManager.AddNewCharacter(Some(_newPlayer));
    }

    pub fn DecreaseUserNum(&mut self) {
        let mut _temp = self.userNum.clone() - 1;
        if(_temp <= 0) {
            self.userNum = 0;
        }
        else {
            self.userNum = _temp;
        }
    }

    pub fn RemovePlayerByID(&mut self, pid : i64)
    {
        self.characterManager.RemoveCharacterByID(pid as usize);
    }

    // 게임로직 내부 전용 호출
    pub fn SendMessageToPerson(&mut self, _sendID : i64, _targetID : i64, _fHeader : FunctionHeader, _fVal : Vec<f64>, _fStr : String) {
        let _somePacket = GamePacket::new(
            _sendID, _targetID,
             _fHeader as i64,
             _fVal , _fStr.to_string() );
        
        SendGamePacket(Some(_somePacket));
    }

    pub fn GameLogicUpate(&mut self) {
        println!("Game Logic Update Test Call");
        self.characterManager.Update();
        // 게임 진행에 필요한 로직들을 업데이트한다.
        // 클라에 반영되어야 하는 내용이 있다면 gSendMessageBuffer에 메세지를 저장한다.
        
        // loop {
        //     thread::sleep(Duration::from_millis(GAME_LOGIC_UPDATE_TICK));
        //     if(false == GetThreadSwitch()) {break;}
        //     
        //     // self.characterManager.Update();
// 
        //     // drop(self);
        //     
        // }
    }

}

