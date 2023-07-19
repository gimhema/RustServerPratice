use std::collections::HashMap;
use std::collections::VecDeque;

use crate::CommonModule;

use crate::CommonModule::Manager::{Manager};

pub struct GameResourceSysManager {
    id : i64
}

impl Manager for GameResourceSysManager {
    fn Initialize(&self) {
        println!("Initailize Game ResourceSys Manager . . . ");
    }
    fn Update(&self) {
        println!("Game ResourceSys Update");
    }
}


