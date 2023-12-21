
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use super::ServerBaseModule::ServerBase;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GamePacket {
    senderID: i64,
    functionHeader: i64,
    functionParam: Vec<f64>
}

impl GamePacket {
    pub fn new(_sendID: i64, _fHeader: i64) -> Self {
        let mut _param = Vec::new();
        GamePacket {
            senderID : _sendID,
            functionHeader : _fHeader,
            functionParam : _param
        }
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
}


impl ServerBase {
    fn GamePacketSerialize(&mut self, packet: &GamePacket) -> Result<String, serde_json::Error> {
        serde_json::to_string(packet)
    } 

    fn GamePacketDeSerialize(&mut self, data: &str) -> Result<GamePacket, serde_json::Error> {
        serde_json::from_str(data)
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
        let game_packet = GamePacket {
            senderID: 42,
            functionHeader: 100,
            functionParam: vec![3.14, 2.71, 1.0],
        };

        let serialized = self.GamePacketSerialize(&game_packet);

        println!("Serialized: {}", serialized.unwrap());
    }

    fn TestDeSerialize(&mut self)
    {
        let game_packet = GamePacket {
            senderID: 77,
            functionHeader: 3,
            functionParam: vec![4.12, 2.88, 3.7],
        };

        let serialized = self.GamePacketSerialize(&game_packet);

        let deserialized = self.GamePacketDeSerialize(&serialized.unwrap());

        let mut _senderId = deserialized.as_ref().unwrap().senderID;
        let mut _functionHeader = deserialized.as_ref().unwrap().functionHeader;
        let mut _funtionParam = deserialized.as_ref().unwrap().functionParam.clone();

        println!("DeSerialized SenderID: {}", _senderId);
        println!("DeSerialized FunctionHeader: {}", _functionHeader);
        
        for fParam in &_funtionParam {
            println!("DeSerialized FunctionParam: {}", fParam);
        }
    }

    pub fn PacketTest(&mut self)
    {
        self.TestSerialize();
        self.TestDeSerialize();
    }
}

