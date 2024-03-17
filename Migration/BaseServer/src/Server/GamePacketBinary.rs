use super::GameCommon::Math::*;
use bincode::{config, Decode, Encode};


#[derive(Encode, Decode, PartialEq, Debug)]
pub struct PacketPlayerTransformation
{
    id : i64,
    location : FLocation,
    rotation : FRotation, 
}
