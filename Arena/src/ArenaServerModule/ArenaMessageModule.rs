
use std::collections::VecDeque;
use std::sync::Mutex;
use mio::{Token};

pub struct ArenaMessage{
    header : Token,
    msg : String
}

impl ArenaMessage {
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
    REQUEST_SEND_ONE,
    REQUEST_SEND_ALL,
    RESPONSE_SEND_ONE,
    RESPONSE_SEND_ALL,
    ERROR,
}

pub fn Initialize() {
    println!("Common Module Initialize . . . .");
}

pub fn ConvertDataToUnique(_data: &str) -> MessageUnique {
    match _data {
        "CONSOLE_WRITE_LINE" => MessageUnique::CONSOLE_WRITE_LINE,
        "NONE" => MessageUnique::NONE,
        "REQUEST_SEND_ONE" => MessageUnique::REQUEST_SEND_ONE,
        "REQUEST_SEND_ALL" => MessageUnique::REQUEST_SEND_ALL,
        "RESPONSE_SEND_ONE" => MessageUnique::RESPONSE_SEND_ONE,
        "RESPONSE_SEND_ONE" => MessageUnique::RESPONSE_SEND_ONE,
        _ => MessageUnique::ERROR
    }
}

pub fn ConvertUniqueToData(_unique: MessageUnique) -> String {
    let mut result: &str = "";
    match _unique {
        MessageUnique::CONSOLE_WRITE_LINE => result = "CONSOLE_WRITE_LINE",
        MessageUnique::REQUEST_SEND_ONE => result = "REQUEST_SEND_ONE",
        MessageUnique::REQUEST_SEND_ALL => result = "REQUEST_SEND_ALL",
        MessageUnique::RESPONSE_SEND_ALL => result = "RESPONSE_SEND_ALL",
        MessageUnique::RESPONSE_SEND_ONE => result = "RESPONSE_SEND_ONE",
        _ => result = "Error",// 
    }
    result.to_string()
}