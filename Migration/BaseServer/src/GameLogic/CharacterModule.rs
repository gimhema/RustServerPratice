use crate::GameCommon::Manager::*;
use crate::GameCommon::Math::*;
use crate::Server::GamePacketBuilder;
use std::collections::VecDeque;
use nalgebra::{Vector3, Quaternion, UnitQuaternion};
use crate::Server::GamePacketBinary::*;
use crate::encode_packet;
use bincode::{config, Decode, Encode};

#[derive(Clone)]
pub enum CharacterType {
    DEFAULT,
    PLAYER,
    ENEMY,
    NPC,
    OBJECT,
}

#[derive(Clone)]
pub struct Character {
    pid : i64,
    name : String,
    location : FLocation,
    rotation : FRotation,
    characterType : CharacterType,
    updateSwitch : bool
}

impl Character {
    pub fn new() -> Character {
        let mut _loc = FLocation::new(0.0, 0.0, 0.0);
        let mut _rot = FRotation::new(
            FEuler::new(0.0, 0.0, 0.0),          
            FQuaternion::new(0.0, 0.0, 0.0, 0.0), 
        );

        Character { pid : -1, name : "".to_string(), location: _loc, rotation: _rot, characterType : CharacterType::DEFAULT, updateSwitch : false }
    }

    pub fn IsUpdated(&mut self) -> bool {
        self.updateSwitch.clone()
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

    pub fn UpdateTransformation(&mut self) {
        //  업데이트 후 전송한다.
        let config = config::standard();
        let _charTransform = PacketPlayerTransformation::new(
            self.GetPID().clone(), self.GetWorldLocation().clone(), self.GetWorldRotation().clone());
        let send_packet = encode_packet!(_charTransform, config);
    }

    pub fn Update(&mut self) {
        println!("Player Update . . .");

        self.UpdateTransformation();

        self.updateSwitch = false;
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

    pub fn SetWorldLocaitonByPID(&mut self, _pID: usize, _new_loc: FLocation) {
        if let Some(player) = self.playerContainer.get_mut(_pID) {
            player.SetWorldLocation(_new_loc);
        }
    }

    pub fn AddNewCharacter(&mut self, newCharacter : Option<Character>) {
        let _newCharacter = newCharacter.unwrap();
        self.playerContainer.push(_newCharacter);
    }

    pub fn GetCharacterByID(&self, index : usize) -> &Character {
        &self.playerContainer[index] 
    }

    pub fn RemoveCharacterByID(&mut self, index : usize) {
        self.playerContainer.remove(index);
    }

    pub fn IsEmptyPlayerContainer(&mut self) -> bool {
        let ret = self.playerContainer.is_empty();
        ret
    }

    pub fn IsEmptyNPCContainer(&mut self) -> bool {
        let ret = self.npcContainer.is_empty();
        ret
    }

    pub fn IsEmptyEnemyContainer(&mut self) -> bool {
        let ret = self.enemyContainer.is_empty();
        ret
    }

    pub fn IsEmptyObjContainer(&mut self) -> bool {
        let ret = self.objContainer.is_empty();
        ret
    }
}

impl Manager for CharacterManager {
    fn Initialize(&mut self)
    {
        println!("Character Manager Initialize . . .");
    }
    fn Update(&mut self)
    {
        println!("Character Manager Update");
        if( false == self.IsEmptyPlayerContainer() )
        {
            for player in &mut self.playerContainer {
                if(player.IsUpdated())
                {
                    player.Update();
                }
            }
        }   
    } 
}