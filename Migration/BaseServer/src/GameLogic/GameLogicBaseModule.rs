
use std::{thread, time};
use std::time::Duration;
use crate::{gSendMessageBuffer, gRecvMessageBuffer, gServer};
use crate::{GetThreadSwitch, SetThreadSwitch};

const GAME_LOGIC_UPDATE_TICK: u64 = 1000;

pub struct GameLogicBase {
    updateCount : i64,
}

impl GameLogicBase {

    pub fn new() -> Self {
        GameLogicBase { updateCount : 0 }
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
        }
    }

}

