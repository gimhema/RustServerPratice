// Manage Instance Game
// Divieded from Arena Server Module . .


use crate::GameLogicCore;
use crate::{sendMessageBuffer, recvMessageBuffer};

use super::ArenaClientModule::ArenaPlayer;
use super::ArenaMessageModule::{self, MakeSendMessage};


pub struct InstanceGame {
    gameID: i32,
    players: Vec<ArenaPlayer>
}

impl InstanceGame {
    // 게임 흐름
    pub fn GameCreate(&mut self) {
        // 게임 생성
    }

    pub fn GameWait(&mut self) {
        // 게임 생성 후 플레이어의 준비 대기
    }

    pub fn GameStart(&mut self) {
        // 게임 시작
    }

    pub fn GameAction(&mut self) {
        // 루프를 돌면서 게임의 로직을 실행
        loop {
            sendMessageBuffer.lock().unwrap().push_back("test".to_string());
        }
    }

    pub fn GameEnd(&mut self) {
        // 게임을 끝내고 결과를 플레이어들에게 전송한다.
    }

    pub fn GameClose(&mut self) {
        // 게임을 닫고 대기상태로 돌아간다.
    }

    pub fn GameDestroy(&mut self) {
        // 인스턴스 게임을 파괴한다.
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

