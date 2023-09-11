use std::collections::VecDeque;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::serverActionMap;

use super::ArenaMessageModule;




pub fn ServerAction_ConsoleWriteLineTest(val : String) -> i64 {
    println!("{}",  val);
    0
}
