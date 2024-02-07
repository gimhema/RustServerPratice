
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use super::ServerBaseModule::ServerBase;
use super::ServerFunctions::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};
use crate::{gServer};

#[derive(Debug, serde::Serialize, serde::Deserialize)]


pub struct GamePacket {
    senderID: i64,
    targetID: i64,
    functionHeader: i64,
    functionParam: Vec<f64>,
    functionStrParam: String,
}

impl GamePacket {
    pub fn new(_sendID: i64, _targetID: i64, _fHeader: i64, _fVec: Vec<f64> ,_fStr: String) -> Self {
        // let mut _param = Vec::new();
        GamePacket {
            senderID : _sendID,
            targetID : _targetID,
            functionHeader : _fHeader,
            functionParam : _fVec,
            functionStrParam : _fStr,
        }
    }

    pub fn getTargetID(&self) -> &i64 {
        &self.targetID
    }

    pub fn getSenderID(&self) -> &i64 {
        &self.senderID
    }

    pub fn getFunctionHeader(&self) -> &i64 {
        &self.functionHeader
    }

    pub fn getFunctionParam(&self) -> &Vec<f64> {
        &self.functionParam
    }

    pub fn getFunctionStrParam(&self) -> &String {
        &self.functionStrParam
    }

    pub fn GamePacketSerialize(packet: &GamePacket) -> Result<String, serde_json::Error> {
        serde_json::to_string(packet)
    }    

    pub fn GamePacketDeSerialize(data: &str) -> Result<GamePacket, serde_json::Error> {
        serde_json::from_str(data)
    }


}


impl ServerBase {
    pub fn GamePacketSerialize(&mut self, packet: &GamePacket) -> Result<String, serde_json::Error> {
        serde_json::to_string(packet)
    } 

    pub fn GamePacketDeSerialize(&mut self, data: &str) -> Result<GamePacket, serde_json::Error> {
        serde_json::from_str(data)
    }

}

pub fn SendGamePacket(packet: Option<GamePacket>) {
    let mut _server = gServer.write().unwrap();
    let send_data = match &packet {
        Some(data) => data,
        None => {
            // Handle the case when packet is None
            return;
        }
    };

    let destination = *send_data.getTargetID();

    let mut _target = Token(0);
    {
        _target = match _server.GetTokenByID(destination) {
            Some(token) => *token,
            None => {
                // Handle the case when GetTokenByID returns None
                return;
            }
        };
    }

    if let Ok(send_msg) = serde_json::to_string(&send_data) {
        let serialized_msg = send_msg.as_bytes();
        if let Some(_targetConn) = _server.GetConnetionByToken(_target) {
            _targetConn.write(serialized_msg);
        }
    }
}




pub struct PacketTestManager {

}

impl PacketTestManager {

    fn GamePacketSerialize(&mut self, packet: &GamePacket) -> Result<String, serde_json::Error> {
        serde_json::to_string(packet)
    } 

    fn GamePacketDeSerialize(&mut self, data: &str) -> Result<GamePacket, serde_json::Error> {
        serde_json::from_str(data)
    }


    fn TestSerialize(&mut self)
    {
        // let game_packet = GamePacket {
        //     senderID: 42,
        //     functionHeader: 100,
        //     functionParam: vec![3.14, 2.71, 1.0],
        // };

        // let serialized = self.GamePacketSerialize(&game_packet);

        // println!("Serialized: {}", serialized.unwrap());
    }

    fn TestDeSerialize(&mut self)
    {
        // let game_packet = GamePacket {
        //     senderID: 77,
        //     functionHeader: 3,
        //     functionParam: vec![4.12, 2.88, 3.7],
        // };

        // let serialized = self.GamePacketSerialize(&game_packet);

        // let deserialized = self.GamePacketDeSerialize(&serialized.unwrap());

        // let mut _senderId = deserialized.as_ref().unwrap().senderID;
        // let mut _functionHeader = deserialized.as_ref().unwrap().functionHeader;
        // let mut _funtionParam = deserialized.as_ref().unwrap().functionParam.clone();

        // println!("DeSerialized SenderID: {}", _senderId);
        // println!("DeSerialized FunctionHeader: {}", _functionHeader);
        
        // for fParam in &_funtionParam {
        //     println!("DeSerialized FunctionParam: {}", fParam);
        // }
    }

    pub fn PacketTest(&mut self)
    {
        self.TestSerialize();
        self.TestDeSerialize();
    }
}

