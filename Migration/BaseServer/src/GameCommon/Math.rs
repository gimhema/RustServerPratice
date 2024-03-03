use nalgebra::{Vector3, Quaternion, UnitQuaternion};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FRotation {
    euler_angles: Vector3<f64>,          // 오일러 각 (Roll, Pitch, Yaw)
    quaternion_angles: Quaternion<f64>,  // 사원수 각    
}

impl FRotation {
    pub fn new(euler_angles: Vector3<f64>, quaternion_angles: Quaternion<f64>) -> Self {
        Self { euler_angles, quaternion_angles }
    }

}

