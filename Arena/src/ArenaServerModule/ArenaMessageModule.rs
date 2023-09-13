use crate::{sendMessageBuffer, recvMessageBuffer};
use std::collections::VecDeque;
use std::sync::Mutex;
use mio::{Token};


// For Recevie.. Need Change Name . . .
pub struct ArenaMessageData {
    uid: i64, // User Identification
    mid: i64, // Message Identification
    value: String, // Message Value
}

impl ArenaMessageData {
    pub fn CreateByMessage(msg: String) -> Self {
        let mut recvMsg = msg;

        // recvMsg를 이리저리 뜯어본다.

        // 뜯은 결과를 메세지로 리턴한다.
        ArenaMessageData { uid: 0, mid: 0, value: "".to_string() } // TEST
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
pub enum MessageUnique {
    CONSOLE_WRITE_LINE,
    NONE,
    ERROR,
}

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
    // gUserContainer for 루프를 돌면서
        // self.PushSendMessageToSendBuffer( pop Token, _command, _param);
}

pub fn SendMessageToOne(_header : Token, _command : i64, _param : String)
{
    // self.PushSendMessageToSendBuffer( _header, _command, _param);
}

    // server_action_map에 저장되어있는 함수들로부터 호출된다
pub fn PushSendMessageToSendBuffer(_header : Token, _command : i64, _param : String) 
{

    // MakeSendMessage(header, command, param)를 만들고
    // sendMsg = "_command:_param"

    let mut sendMsg = ArenaMessage::new(_header, _param);

    sendMessageBuffer.lock().unwrap().push_back(sendMsg);
}