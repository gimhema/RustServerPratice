use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::Manager::{Manager};


pub struct GameNPCManager {
    id : i64
}

impl Manager for GameNPCManager {
    fn Initialize(&self) {
        println!("Initailize Game NPC Manager . . . ");
    }
    fn Update(&self) {
        println!("Game NPC Manager Update");
    }
}

pub struct GameNonPlayerableEcoSystem {
    id : i64
}

impl Manager for GameNonPlayerableEcoSystem {
    fn Initialize(&self) {
        println!("Initailize Game NonPlayerable EcoSystem . . . ");
    }
    fn Update(&self) {
        println!("Game NonPlayerable EcoSystem Update");
    }
}



