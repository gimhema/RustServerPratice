use super::GameCommon::Math::*;
use bincode::{config, Decode, Encode};


#[derive(Encode, Decode, PartialEq, Debug)]
pub struct PacketPlayerTransformation
{
    id : i64,
    location : FLocation,
    rotation : FRotation, 
}

impl PacketPlayerTransformation {
    pub fn new(_id : i64, _location : FLocation, _rotation : FRotation) -> PacketPlayerTransformation {
        PacketPlayerTransformation{id : _id, location : _location, rotation : _rotation}
    }
}

