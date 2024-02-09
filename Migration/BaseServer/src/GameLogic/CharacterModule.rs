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

    pub fn Update(&mut self) {

    }
}


pub struct CharacterManager {
    characterContainer : Vec<Character>
}

impl CharacterManager {
    pub fn new() -> CharacterManager {
        CharacterManager { characterContainer: Vec::new() }
    }

    pub fn AddNewCharacter(&mut self, newCharacter : Option<Character>) {
        let _newCharacter = newCharacter.unwrap();
        self.characterContainer.push(_newCharacter);
    }

    pub fn GetCharacterByID(&mut self, index : usize) -> &Character {
        &self.characterContainer[index] 
    }

}

impl Manager for CharacterManager {
    fn Initialize(&self)
    {

    }
    fn Update(&self)
    {

    } 
}