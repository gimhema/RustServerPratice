use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use super::ServerBaseModule::ServerBase;
use super::ServerFunctions::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::io::{self, Read, Write};
use crate::{gServer};
use crate::GetGameLogic;


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

pub fn SendGamePacketToConnect(packet: Option<GamePacket>, mut connecStream : TcpStream)
{
    println!("Send Game Message Call 1 1");

    let send_data = match &packet {
        Some(data) => {
            println!("Send Game Message Valid");
            data
        },
        None => {
            // Handle the case when packet is None
            println!("Send Game Message Exception 1");
            return;
        }
    };

    if let Ok(send_msg) = serde_json::to_string(&send_data) {
        let serialized_msg = send_msg.as_bytes();
            println!("Send Game Message {}", send_msg);
            connecStream.write(serialized_msg);
    }
    println!("Send Game Message Call 1 1 END");
}

pub fn SendGamePacket(packet: Option<GamePacket>) {
    let send_data = match &packet {
        Some(data) => {
            println!("Send Game Message Valid");
            data
        },
        None => {
            // Handle the case when packet is None
            println!("Send Game Message Exception 1");
            return;
        }
    };

    let destination = *send_data.getTargetID();

    let mut _target = Token(0);
    {
        _target = match GetGameLogic().write().unwrap().GetUserTokenByID(destination) {
            Some(token) => *token,
            None => {
                // Handle the case when GetTokenByID returns None
                // println!("Send Game Message Exception 2");
                return;
            }
        };
    }

    if let Ok(send_msg) = serde_json::to_string(&send_data) {
        let serialized_msg = send_msg.as_bytes();
        if let Some(_targetConn) = GetGameLogic().write().unwrap().GetUserConnectionsByToken(_target) {
            println!("Send Game Message {}", send_msg);
            _targetConn.write(serialized_msg);
        }
    }

    println!("Send Game Message Call End");
}

pub fn SendGamePacketAllUser(packet : Option<GamePacket>) {
    let send_data = match &packet {
        Some(data) => {
            println!("Send Game Message Valid");
            data
        },
        None => {
            // Handle the case when packet is None
            println!("Send Game Message Exception 1");
            return;
        }
    };

    for (key, value) in GetGameLogic().write().unwrap().GetUserTokenMap() {

        if let Ok(send_msg) = serde_json::to_string(&send_data) {
            let serialized_msg = send_msg.as_bytes();
            if let Some(_targetConn) = GetGameLogic().write().unwrap().GetUserConnectionsByToken(*value) {
                println!("Send Game Message {}", send_msg);
                _targetConn.write(serialized_msg);
            }
        }

    }
}

