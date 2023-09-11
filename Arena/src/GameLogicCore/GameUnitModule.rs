use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::GameActor::Actor;
use crate::CommonModule::GameActor::ActorStatus;
use crate::CommonModule::Manager::{Manager};
use crate::CommonModule::GameActor;

pub struct GameUnitManager {
    id : i64,
    gameUnitContainer : Vec<GamePlayerUnit>
}

impl GameUnitManager {
    pub fn new(gid: i64) -> Self {
        // init gameUnitContainer
        let game_unit_container = vec![];
        GameUnitManager {
            id: gid,
            gameUnitContainer: game_unit_container,
        }
    }
}


impl Manager for GameUnitManager {
    fn Initialize(&self) {
        println!("Initailize Game Unit Manager . . . ");
    }
    fn Update(&self) {
        println!("Game Unit Manager Update");
       // Units Status Update By Player(=Client)'s Action

       // Get Units Status

        // and push Units Status to Send Buffer

    }
}

pub struct GamePlayerUnit {
    id : i64,
    UnitStatus : ActorStatus
}

impl Actor for GamePlayerUnit {
    fn Create(&self) {
        
    }
    
    fn Update(&self) {
        
    }
}















