use super::GameCommon::Math::*;
use bincode::{config, Decode, Encode};


#[derive(Encode, Decode, PartialEq, Debug)]
pub struct PacketPlayerTransformation
{
    id : i64,
    location : FLocation,
//    rotation : FRotation, 회전 구조체는 나중에 재정의
}
