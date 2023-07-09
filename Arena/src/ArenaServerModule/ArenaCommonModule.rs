
use std::collections::VecDeque;
use std::sync::Mutex;

static sendMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
static recvMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());


pub enum MessageUnique {
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
        MessageUnique::NONE => result = "NONE",
        MessageUnique::REQUEST_SEND_ONE => result = "REQUEST_SEND_ONE",
        MessageUnique::REQUEST_SEND_ALL => result = "REQUEST_SEND_ALL",
        MessageUnique::RESPONSE_SEND_ALL => result = "RESPONSE_SEND_ALL",
        MessageUnique::RESPONSE_SEND_ONE => result = "RESPONSE_SEND_ONE",
        _ => result = "Error",// 
    }
    result.to_string()
}