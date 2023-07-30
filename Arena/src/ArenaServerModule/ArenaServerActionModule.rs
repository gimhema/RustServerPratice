use std::collections::VecDeque;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::serverActionMap;

use super::ArenaMessageModule;


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


pub fn InitServerActions() {
    // Push server action functions to action map . . .
    println!("Init Server Action Functions . . .");
}

pub fn ServerActionTest(some: String) {
//    serverActionMap가지고 무언갈 한다 . . .
    serverActionMap.lock().unwrap().get("Some").unwrap();
}
