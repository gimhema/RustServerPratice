use super::ServerFunctions::{FunctionParam, FunctionHeader};
use crate::{gRecvMessageBuffer, gSendMessageBuffer};
use crate::{gServer};

pub fn ServerAction_CHAT_MESSAGE_ALL(val : FunctionParam) -> i64 {
    // Send to All User
    let sendMsg = val.getFunctionStrParam();
    0
}

pub fn ServerAction_CHAT_MESSAGE_TO_ONE(val : FunctionParam) -> i64 {

    0
}

pub fn ServerAction_CHAT_MESSAGE_TO_GROUP(val : FunctionParam) -> i64 {

    0
}