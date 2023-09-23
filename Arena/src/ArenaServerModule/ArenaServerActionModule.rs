use std::collections::VecDeque;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::serverActionMap;

use super::ArenaMessageModule;




pub fn ServerAction_ConsoleWriteLineTest(val : String) -> i64 {
    println!("{}",  val);
    0
}

pub fn ServerAction_MessageTestFunction2(val : String) -> i64 {
    println!("Message Test 2 {}",  val);
    0
}

pub fn ServerAction_MessageTestFunction3(val : String) -> i64 {
    println!("Message Test 3 {}",  val);
    0
}
