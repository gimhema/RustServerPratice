
use std::{thread, time};
use std::time::Duration;
use crate::{gSendMessageBuffer, gRecvMessageBuffer, gServer};
use crate::{GetThreadSwitch, SetThreadSwitch};
use crate::Server::GamePacketModule::*;
use crate::Server::ServerFunctions::*;

const GAME_LOGIC_UPDATE_TICK: u64 = 1000;
const MAX_NUM_USER: i64 = 2;

pub struct GameLogicBase {
    updateCount : i64,
    userNum : i64,
}

impl GameLogicBase {

    pub fn new() -> Self {
        GameLogicBase { updateCount : 0, userNum : 0 }
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

    pub fn DecreaseUserNum(&mut self) {
        let mut _temp = self.userNum.clone() - 1;
        if(_temp <= 0) {
            self.userNum = 0;
        }
        else {
            self.userNum = _temp;
        }
    }

    pub fn GameLogicUpate(&mut self) {
        println!("Game Logic Update Start ! ! !");

        // 게임 진행에 필요한 로직들을 업데이트한다.
        // 클라에 반영되어야 하는 내용이 있다면 gSendMessageBuffer에 메세지를 저장한다.
        loop {
            thread::sleep(Duration::from_millis(GAME_LOGIC_UPDATE_TICK));
            if(false == GetThreadSwitch()) {break;}
            println!("Game Logic Update . . . update count : {}", self.updateCount);
            self.updateCount += 1;

            // TEST
            // let _val = Vec<f64>;
            let _somePacket = GamePacket::new(
                0, 1,
                 FunctionHeader::CHAT_MESSAGE_ALL as i64,
                 vec![1.0, 2.5, 3.7] , "Hello All Clients!".to_string());

            SendGamePacket(Some(_somePacket));
        }
    }

}

