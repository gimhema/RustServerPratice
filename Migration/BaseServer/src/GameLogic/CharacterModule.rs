use crate::GameCommon::Manager::*;
use crate::GameCommon::Math::*;
use std::collections::VecDeque;

pub struct Character {
    pid : i64,
    name : String,
    location : FLocation,
    rotation : FRotation
}

impl Character {
    pub fn new() -> Character {
        let mut _loc = FLocation::new(0.0, 0.0, 0.0);
        let mut _rot = FRotation::new(0.0, 0.0, 0.0, 0.0, 0.0, 
        0.0, 0.0);

        Character { pid : -1, name : "".to_string(), location: _loc, rotation: _rot }
    }

    pub fn GetPID(&mut self) -> &i64 {
        &self.pid
    }

    pub fn GetName(&mut self) -> &String {
        &self.name
    }

    pub fn GetWorldLocation(&mut self) -> &FLocation {
        &self.location
    }

    pub fn GetWorldRotation(&mut self) -> &FRotation {
        &self.rotation
    }

    pub fn SetPID(&mut self, _pid : i64) {
        self.pid = _pid;
    }

    pub fn SetName(&mut self, _name : String) {
        self.name = _name;
    }

    pub fn SetWorldLocation(&mut self, _location : FLocation) {
        self.location = _location;
    }

    pub fn SetWorldRotation(&mut self, _rotation : FRotation) {
        self.rotation = _rotation;
    }

    pub fn Update(&mut self) {

    }
}


pub struct CharacterManager {
    playerContainer : Vec<Character>,
    npcContainer : Vec<Character>,
    enemyContainer : Vec<Character>,
    objContainer : Vec<Character>
}

impl CharacterManager {
    pub fn new() -> CharacterManager {
        CharacterManager { 
            playerContainer: Vec::new(),
            npcContainer: Vec::new(),
            enemyContainer: Vec::new(),
            objContainer : Vec::new()
         }
    }

    pub fn AddNewCharacter(&mut self, newCharacter : Option<Character>) {
        let _newCharacter = newCharacter.unwrap();
        self.playerContainer.push(_newCharacter);
    }

    pub fn GetCharacterByID(&mut self, index : usize) -> &Character {
        &self.playerContainer[index] 
    }

    pub fn RemoveCharacterByID(&mut self, index : usize) {
        self.playerContainer.remove(index);
    }

    pub fn IsEmptyPlayerContainer(&mut self) -> bool {
        let ret = self.playerContainer.is_empty();

        ret
    }
}

impl Manager for CharacterManager {
    fn Initialize(&mut self)
    {

    }
    fn Update(&mut self)
    {
        println!("Character Manager Update");
        if( false == self.IsEmptyPlayerContainer() )
        {
            
        }    
    } 
}