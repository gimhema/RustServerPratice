use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::Manager::{Manager};

pub struct GameUnitManager {
    id : i64
}


impl Manager for GameUnitManager {
    fn Initialize(&self) {
        println!("Initailize Game Unit Manager . . . ");
    }
    fn Update(&self) {
        println!("Game Unit Manager Update");
    }
}