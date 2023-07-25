use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::GameActor::Actor;
use crate::CommonModule::GameActor::ActorStatus;
use crate::CommonModule::Manager::AutoActionManager;
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
    id : i64,
    NPCContainer : Vec<GameNPCUnit>
}

impl Manager for GameNPCManager {
    fn Initialize(&self) {
        println!("Initailize Game NPC Manager . . . ");
    }
    fn Update(&self) {
        println!("Game NPC Manager Update");

        // NPC is auto action

        // Get NPC's Status, and push status info to send buffer

    }
}

impl AutoActionManager for GameNPCManager {
    fn AutoUpdate(&self) {
        println!("Game NPC Manager Auto Update")

        // Update NPC status auto
    }
}


pub enum AtmosphereType {
    NONE,
    DAY,
    NIGHT,
}

pub struct GameNonPlayerableEcoSystem {
    id : i64,
    currentAtmosphere : AtmosphereType
}

impl Manager for GameNonPlayerableEcoSystem {
    fn Initialize(&self) {
        println!("Initailize Game NonPlayerable EcoSystem . . . ");
    }
    fn Update(&self) {
        println!("Game NonPlayerable EcoSystem Update");

        // EcoSystem is auto action

        // Get Ecosystem Status, and push status info to send buffer        
    }
}

impl AutoActionManager for GameNonPlayerableEcoSystem {
    fn AutoUpdate(&self) {
        println!("Game EcoSystem Auto Update")

        // Update NPC status auto
    }
}











