
use std::collections::VecDeque;
use std::sync::Mutex;



pub struct ArenaMessage{
    header : i64,
    msg : String
}

// 콜백 함수 및 하나의 게임 로직이 끝난 뒤 결과를 sendBuffer에 저장하기전에 실행하는 함수
pub fn MakeSendMessage(_header : i64, _command : i64, _param : String) -> ArenaMessage {

    // _header : 누구한테 보낼거냐
    // _command : 클라한테 어떤 명령을 내릴거냐
    // _param : 명령의 내용은 어떤 내용이냐

    // 이걸로 String 타입의 메세지를 만들고 리턴한다.
    let mut result: ArenaMessage = ArenaMessage {
        header : 0,
        msg : "string".to_string(),
    };
    
    return result;
}

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