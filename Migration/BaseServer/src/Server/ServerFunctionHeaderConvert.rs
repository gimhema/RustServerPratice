
use super::ServerFunctions::*;

impl From<FunctionHeader> for i64 {
    fn from(header: FunctionHeader) -> Self {
        match header {
            FunctionHeader::DEFAULT => 0,
            FunctionHeader::CHAT_MESSAGE_ALL => 1,
            FunctionHeader::CHAT_MESSAGE_TO_ONE => 2,
            FunctionHeader::CHAT_MESSAGE_TO_GROUP => 3,
        }
    }
}

impl From<i64> for FunctionHeader {
    fn from(header: i64) -> Self {
        match header {
            0 => FunctionHeader::DEFAULT,
            1 => FunctionHeader::CHAT_MESSAGE_ALL,
            2 => FunctionHeader::CHAT_MESSAGE_TO_ONE,
            3 => FunctionHeader::CHAT_MESSAGE_TO_GROUP,
            i64::MIN..=-1_i64 | 4_i64..=i64::MAX => todo!(),
        }
    }
}

