
use super::ServerFunctions::*;

impl From<FunctionHeader> for i64 {
    fn from(header: FunctionHeader) -> Self {
        match header {
            FunctionHeader::CHAT_MESSAGE_ALL => 0,
            FunctionHeader::CHAT_MESSAGE_TO_ONE => 1,
            FunctionHeader::CHAT_MESSAGE_TO_GROUP => 2,
        }
    }
}
