// Handling Client's Information

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;
use mio::net::{TcpListener, TcpStream};
use mio::{Token};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::GameLogicCore::GameLogicCore::GamePlayerLogic;
use crate::sendMessageBuffer;
// use crate::someGlobal;
use crate::GameLogicCore;

use crate::gUserContainer;


const SENDTEST: &[u8] = b"Hi Unreal This Message Is Update Loop\n";

pub fn SendTestFunction(connectionStream: &mut TcpStream)
{
    connectionStream.write(SENDTEST);
}

pub fn Initailize() {
    println!("Arena Client Initialize");
}

pub struct ArenaPlayer {
    userID: i64,
    userName: String,
    playerLogic: GamePlayerLogic,
    userToken: Token
}


impl  ArenaPlayer {
    pub fn new(gid: i64, _name: String, _token: Token) -> Self {
        let game_player_logic = GamePlayerLogic::new(gid);

        ArenaPlayer { 
            userID: gid,
            userName: _name,
            playerLogic: game_player_logic,
            userToken: _token 
        }
    }

    pub fn AutoUpdate(&mut self) {
        // PlayerLogic 일부 요소들을 자동으로 업데이트한다.
        // 생산 정보, 건물 정보, 시간등과 관련된 . . . .
    }

    pub fn Initialize(&mut self, ID: i64, userName: String, IP: String) {
        self.userID = ID;
        self.userName = userName;
    }

    pub fn get_userID(&self) -> &i64 {
        &self.userID
    }

    pub fn get_userName(&self) -> &String {
        &self.userName
    }

    pub fn get_playerLogic(&self) -> &GamePlayerLogic {
        &self.playerLogic
    }

    pub fn get_userToken(&self) -> &Token {
        &self.userToken
    }

    pub fn set_userID(&mut self, new_id: i64) {
        self.userID = new_id;
    }

    pub fn set_userName(&mut self, new_name: String) {
        self.userName = new_name;
    }

    pub fn set_playerLogic(&mut self, new_logic: GamePlayerLogic) {
        self.playerLogic = new_logic;
    }

    pub fn set_userToken(&mut self, new_token: Token) {
        self.userToken = new_token;
    }
}

pub fn AddNewUserToContainer(userIndex: i64, userToken: Token, userName: String)
{
    // gUserContainer에 새로운 유저를 추가한다.

    let new_arena_player = ArenaPlayer::new(userIndex, userName, userToken);

    gUserContainer.lock().unwrap().insert(userToken, new_arena_player);
}

