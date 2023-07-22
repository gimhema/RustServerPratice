use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::GameActor::Actor;
use crate::CommonModule::GameActor::ActorStatus;
use crate::CommonModule::Manager::{Manager};
use crate::CommonModule::FSM;
use crate::CommonModule::GameActor;

pub struct GameNPCUnit {
    id : i64,
    NPCStatus : ActorStatus
}

impl Actor for GameNPCUnit {
    fn Create(&self) {
        
    }
    fn Update(&self) {
        
    }
}

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



