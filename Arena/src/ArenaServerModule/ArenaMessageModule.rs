use crate::{sendMessageBuffer, recvMessageBuffer};
use std::collections::VecDeque;
use std::sync::Mutex;
use mio::{Token};
use std::collections::HashMap;
use std::sync::{MutexGuard};
use crate::gUserContainer;
use super::ArenaClientModule::ArenaPlayer;
use super::ArenaMessageUnique::MessageUnique;

// For Recevie.. Need Change Name . . .
pub struct ArenaMessageData {
    uid: i64, // User Identification
    mid: i64, // Message Identification
    value: String, // Message Value
}

impl ArenaMessageData {
    pub fn CreateByMessage(msg: String) -> Self {
        let mut recv_msg = msg;
        
        let parts: Vec<&str> = recv_msg.split(':').collect();

        let uid = parts.get(0).unwrap_or(&"0").parse().unwrap_or(0);
        let mid = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
        let value = parts.get(2).unwrap_or(&"").to_string();

        ArenaMessageData { uid, mid, value }
    }

    pub fn get_uid(&self) -> &i64 {
        &self.uid
    }
    pub fn get_mid(&self) -> &i64 {
        &self.mid
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }
}


// To Send.. Need Change Name . . .
pub struct ArenaMessage{
    header : Token,
    msg : String
}

impl ArenaMessage {
    pub fn new(_header: Token, _msg: String) -> Self {
        ArenaMessage { header: _header, msg: _msg }
    }

    pub fn get_header(&self) -> &Token {
        &self.header
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}
// pub enum MessageUnique {
//     CONSOLE_WRITE_LINE,
//     TEST_MESSAGE2,
//     TEST_MESSAGE3,
//     NONE,
//     ERROR,
// }

pub fn Initialize() {
    println!("Common Module Initialize . . . .");
}

pub fn ConvertDataToUnique(_data: &str) -> MessageUnique {
    match _data {
        "CONSOLE_WRITE_LINE" => MessageUnique::CONSOLE_WRITE_LINE,
        "NONE" => MessageUnique::NONE,
        _ => MessageUnique::ERROR
    }
}

pub fn ConvertUniqueToData(_unique: MessageUnique) -> String {
    let mut result: &str = "";
    match _unique {
        MessageUnique::CONSOLE_WRITE_LINE => result = "CONSOLE_WRITE_LINE",
        _ => result = "Error",// 
    }
    result.to_string()
}

pub fn SendMessageToAll( _command : i64, _param : String)
{

        let lock: MutexGuard<HashMap<Token, ArenaPlayer>> = gUserContainer.lock().unwrap();
    

        for (token, player) in lock.iter() {
            PushSendMessageToSendBuffer( *token, _command, _param.clone());
        }

}

pub fn SendMessageToOne(_header : Token, _command : i64, _param : String)
{
    PushSendMessageToSendBuffer( _header, _command, _param);
}

    // server_action_map에 저장되어있는 함수들로부터 호출된다
pub fn PushSendMessageToSendBuffer(_header : Token, _command : i64, _param : String) 
{

    // MakeSendMessage(header, command, param)를 만들고
    // sendMsg = "_command:_param"
    let mut _data = format!("{}:{}", _command, _param);

    let mut sendMsg = ArenaMessage::new(_header, _data);

    sendMessageBuffer.lock().unwrap().push_back(sendMsg);
}