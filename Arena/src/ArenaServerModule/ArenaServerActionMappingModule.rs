// ServerAction 함수와 메세지를 매핑한다.
use std::collections::VecDeque;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::serverActionMap;

use super::ArenaMessageModule;
use super::ArenaMessageModule::MessageUnique;
use super::ArenaServerActionModule::{self, ServerAction_ConsoleWriteLineTest};


pub fn ServerActionMappingInitialize() {
    // serverActionMap을 초기화한다.

    println!("Initialize Server Action Mapping . . . .");

    serverActionMap.lock().unwrap().insert(
        MessageUnique::CONSOLE_WRITE_LINE as i64,
         ServerAction_ConsoleWriteLineTest);
    
}

pub fn CallServerAction(uid: i64, mid: i64, value: String) {
    //    serverActionMap가지고 무언갈 한다 . . .
        serverActionMap.lock().unwrap().get(&mid).unwrap();
}


