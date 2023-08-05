// ServerAction 함수와 메세지를 매핑한다.
use std::collections::VecDeque;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::serverActionMap;

use super::ArenaMessageModule;
use super::ArenaMessageModule::MessageUnique;
use super::ArenaServerActionModule::{self, ServerAction_ConsoleWriteLineTest};

// Function Pointer Example
// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
// let mut x = add(5,7);
// type Binop = fn(i32, i32) -> i32;
// let bo: Binop = add;
// x = bo(5,7);

// Function map structure . . .
// Key : Command
// Value : Functions by Command
// Value Functions Param : Param from message


pub fn ServerActionMappingInitialize() {
    // serverActionMap을 초기화한다.

    serverActionMap.lock().unwrap().insert(
        MessageUnique::CONSOLE_WRITE_LINE as i64,
         ServerAction_ConsoleWriteLineTest);
    
}

pub fn CallServerAction(_key: i64) {
    //    serverActionMap가지고 무언갈 한다 . . .
        serverActionMap.lock().unwrap().get(&_key).unwrap();
}


