use nalgebra::{Vector3, Quaternion, UnitQuaternion};
use bincode::{config, Decode, Encode};

#[derive(Clone, Encode, Decode, PartialEq, Debug)]
pub struct FLocation {
    x : f64,
    y : f64,
    z : f64
}

impl FLocation {
    pub fn new( _x : f64, _y : f64, _z : f64) -> FLocation {
        FLocation { x: _x, y: _y, z: _z }
    }

    pub fn MakeLocationByVec(_v : Vec<f64>) -> FLocation {
        FLocation::new(_v[0], _v[1], _v[1])
    }

    pub fn GetX(&mut self) -> &f64 {
        &self.x
    }

    pub fn GetY(&mut self) -> &f64 {
        &self.y
    }

    pub fn GetZ(&mut self) -> &f64 {
        &self.z
    }

    pub fn SetX(&mut self, _x : f64) {
        self.x = _x;
    }

    pub fn SetY(&mut self, _y : f64) {
        self.y = _y;
    }

    pub fn SetZ(&mut self, _z : f64) {
        self.z = _z;
    }
}

#[derive(Copy, Clone, Encode, Decode, PartialEq, Debug)]
pub struct FEuler
{
    roll : f64,
    pitch : f64,
    yaw : f64
}

impl FEuler {
    pub fn new(roll : f64, pitch : f64, yaw : f64) -> Self {
        Self { roll, pitch, yaw }
    }
}

#[derive(Copy, Clone, Encode, Decode, PartialEq, Debug)]
pub struct FQuaternion
{
    x : f64,
    y : f64,
    z : f64,
    w : f64
}

impl FQuaternion {
    pub fn new (x : f64, y : f64, z : f64, w : f64) -> Self {
        Self{ x, y, z, w }
    }

    pub fn euler_to_quaternion(euler: FEuler) -> Self {
        // euler 기반으로 계산
        FQuaternion::new(0.0, 0.0, 0.0, 0.0)
    }
}


#[derive(Clone, Encode, Decode, PartialEq, Debug)]
pub struct FRotation {
    euler_angles: FEuler,          // 오일러 각 (Roll, Pitch, Yaw)
    quaternion_angles: FQuaternion,  // 사원수 각    
}

impl FRotation {
    pub fn new(euler_angles: FEuler, quaternion_angles: FQuaternion) -> Self {
        Self { euler_angles, quaternion_angles }
    }

    pub fn new_by_euler(euler: FEuler) -> Self {
        let converted_quaternion = FQuaternion::euler_to_quaternion(euler);
        Self{euler_angles : euler.clone(), quaternion_angles : converted_quaternion}
    }



}

