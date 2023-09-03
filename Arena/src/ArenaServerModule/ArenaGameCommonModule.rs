// Manage Instance Game
// Divieded from Arena Server Module . .


use crate::GameLogicCore;
use crate::{sendMessageBuffer, recvMessageBuffer};
use crate::gUserContainer;
use crate::serverActionMap;

use super::ArenaClientModule::ArenaPlayer;
use super::ArenaMessageModule::{self};


pub struct InstanceGame {
    gameID: i64,
    num_player: i64,
    max_num_player: i64,
    isStart: bool,
    isGameConclusion: bool
}


impl InstanceGame {
    // 게임 흐름
    pub fn new(gid: i64, num_player: i64, max_num_player: i64) -> Self {
        InstanceGame { gameID: gid, num_player: num_player, max_num_player: max_num_player, isStart: false, isGameConclusion: false }
    }

    pub fn get_gameID(&self) -> &i64 {
        &self.gameID
    }

    pub fn get_num_player(&self) -> &i64 {
        &self.num_player
    }

    pub fn get_max_num_player(&self) -> &i64 {
        &self.max_num_player
    }

    pub fn IsStart(&self) -> bool {
        self.isStart
    }

    pub fn SetStartSwitch(&mut self, _switch: bool) {
        self.isStart = _switch;
    }

    pub fn set_gameID(&mut self, gID: i64) {
        self.gameID = gID;
    }

    pub fn set_num_player(&mut self, num_player: i64) {
        self.num_player = num_player;
    }

    pub fn set_max_num_player(&mut self, max_num_player: i64) {
        self.max_num_player = max_num_player;
    }

    pub fn IsFull(&mut self) -> bool {

        let playerNum: i64 = gUserContainer.lock().unwrap().len() as i64;
        let mut result = false;
        if (playerNum == self.max_num_player) {
            result = true
        }
        result
    }

    pub fn GameReset(&mut self) {
        // 게임 시작 전에 리셋
        println!("Game Reset");
    }

    pub fn IncreasePlayerCount(&mut self) {
        self.set_num_player(self.num_player + 1);
    }

    pub fn GameStart(&mut self) {
        // 게임 시작
        println!("Game Start");
        self.GameReset();
        self.GameAction();
    }

    pub fn GameWait(&mut self) {
        loop {
            // User Container 상태를 보고 IsStart를 결정한다.
            if( self.IsFull() )
            {
                break;
            }
        }
        self.GameStart();              
    }

    pub fn GameAction(&mut self) {
        loop {
            if(self.isGameConclusion == true)
            {
                break;
            }
//            sendMessageBuffer.lock().unwrap().push_back("test".to_string());
        }
        self.GameEnd();
    }

    pub fn GameEnd(&mut self) {
        // 게임을 끝내고 결과를 플레이어들에게 전송한다.
        println!("Game End . . . .");
        self.GameUserOut();
    }
    
    pub fn GameUserOut(&mut self) {
        // 전송 후 접속한 유저들을 아웃시킨다.
        println!("Start User Out Task . . .");
    }

    // Async
    // InstanceGame의 GameWait 이후 시작
    pub fn RecvMessageProcessLoop(&mut self) {
        loop {
            // recvMessageBuffer.lock().unwrap() 에서 메세지를 꺼낸다.
                let msg = recvMessageBuffer.lock().unwrap().pop_back();
            // 메세지를 꺼내고 헤더에 맞는 콜백 함수를 실행시킨다.

            // 콜백함수에는 헤더 이후에 작성된 정보들이 저장되어있다.
        
        }
    }

    pub fn PushSendMessageToSendBuffer(&mut self, _header : i64, _command : i64, _param : String) {

//        MakeSendMessage(header, command, param)를 만들고
        // sendBuffer에 메세지를 저장한다.

    }
}

