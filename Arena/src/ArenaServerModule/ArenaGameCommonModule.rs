// Manage Instance Game
// Divieded from Arena Server Module . .
// sendMessageBuffer.lock().unwrap().push_back("test".to_string());

use crate::CommonModule::Manager::Manager;
use crate::GameLogicCore;
use crate::GameLogicCore::GameLogicCore::GameNonPlayerbleSystem;
use crate::{sendMessageBuffer, recvMessageBuffer};
use crate::gUserContainer;
use crate::serverActionMap;
use mio::{Token};
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;
use std::{thread, time};

use super::ArenaClientModule::ArenaPlayer;
use super::ArenaMessageModule::{self, ArenaMessageData, ArenaMessage};
use super::CommonModule::GameActionThreadPool::ThreadPool;

const THREAD_POOL_CAPACITY: usize = 10;

pub struct InstanceGame {
    gameID: i64,
    num_player: i64,
    max_num_player: i64,
    isStart: bool,
    isGameConclusion: bool,
    nonPlayerbleSystem : GameNonPlayerbleSystem,
    gameActionThreadPool : ThreadPool,
    isConclude: bool
}


impl InstanceGame {
    // 게임 흐름
    pub fn new(gid: i64, num_player: i64, max_num_player: i64) -> Self {
        let _nonPlyayerbleSystem = GameNonPlayerbleSystem::new(gid);
        let _threadPool = ThreadPool::new(THREAD_POOL_CAPACITY);
        InstanceGame { 
            gameID: gid,
            num_player: num_player,
            max_num_player: max_num_player, 
            isStart: false, 
            isGameConclusion: false, 
            nonPlayerbleSystem: _nonPlyayerbleSystem,
            gameActionThreadPool: _threadPool,
            isConclude: false
         }
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

    pub fn IsConclude(&self) -> bool {
        self.isConclude
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


    pub fn GameStart(&mut self) {
        // 게임 시작
        println!("Game Start");
        self.isStart = true;
        self.GameReset();
    }


    pub fn GameLogicUpdate(&mut self) {

        self.GameWait();

        self.GameStart();


    }

    pub fn GameWait(&mut self) {
        let ten_millis = time::Duration::from_millis(1000);

        loop {
            // User Container 상태를 보고 IsStart를 결정한다.
            thread::sleep(ten_millis);
            println!("Game Wait . . .");
            if( self.IsFull() )
            {
                break;
            }
        }
        println!("Game Ready ! !");
    }

    pub fn GamePlayerAutoLogicUpdate(&mut self) {
        loop {
            println!("Check Player Auto Update . . .");
            if (true == self.IsStart())
            {
                let mut container_lock: MutexGuard<HashMap<Token, ArenaPlayer>> = gUserContainer.lock().unwrap();
                if (true == self.IsConclude()) { break; }

                // HashMap의 원소를 순회하면서 AutoUpdate 메서드를 호출합니다.
                for (_, player) in container_lock.iter_mut() {
                    player.AutoUpdate(); // 메서드 호출
                }
                // Mutex 락을 해제합니다.
                drop(container_lock);
            }
        }
    }


    pub fn CheckGameStatus(&mut self){
        // 게임의 승패를 판정한다.
        loop 
        {
            println!("Check Game Status . . . .");
            // TEST
            // self.SetStartSwitch(true);
            if (true == self.IsStart())
            {
                if (true == self.IsConclude()) { break; }
            }
        }
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
        loop
        {
            println!("Loop Recv Message . . .");
            if( true == self.IsStart() )
            {
                if (self.isGameConclusion == true) { break; } // 게임이 끝났다면 종료
                // recvMessageBuffer.lock().unwrap() 에서 메세지를 꺼낸다. (main.rs 275~290 line)
                let msg = recvMessageBuffer.lock().unwrap().pop_back();
                // "uid:mid:mVal" 형식으로 받아올것이다.
                let mut data = ArenaMessageData::CreateByMessage(msg.unwrap());
                // 메세지를 꺼내서 이리저리 뜯어본다.
                let mut uid = data.get_uid(); // User ID
                let mut mid = data.get_mid(); // Message ID
                let mut mVal = data.get_value(); // Message Function
        
                // 콜백함수에는 헤더 이후에 작성된 정보들이 저장되어있다.
                self.CallServerAction(uid, mid, mVal);
            }
        }
    }

    pub fn CallServerAction(&mut self, userID : &i64, funcID : &i64, funcParam : &String ) {

        let server_action_map = &*serverActionMap.lock().unwrap();

        // call server_action_map[funcID](funcParam) 
        if let Some(server_action) = server_action_map.get(funcID) {
        let result = server_action(funcParam.clone());

        println!("Result: {}", result);
        } else {
            println!("Function not found");
        }
    }

    pub fn GameNonPlayerAction(&mut self) {
        loop
        {
            println!("Check NonPlayer Logic in loop . .");
            if (true == self.IsStart())
            {
                if (self.isGameConclusion == false)
                {
                    self.nonPlayerbleSystem.Update();
                }
            }
        }
    }
}

