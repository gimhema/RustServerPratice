use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::Manager::{Manager};

pub struct GameStructureManager {
    id : i64
}


impl Manager for GameStructureManager {
    fn Initialize(&self) {
        println!("Initailize Game Structure Manager . . . ");
    }
    fn Update(&self) {
        println!("Game Structure Update");
       // Structure Status Update By Player(=Client)'s Action

        // Get Status of Game Structure Object

        // and Push Status of Game Structure Object to Send Buffer

    }
}

