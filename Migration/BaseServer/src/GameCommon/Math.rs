

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


pub struct FRotation {
    roll : f64,
    pitch : f64,
    yaw : f64,
    x : f64,
    y : f64,
    z : f64,
    w : f64
}

impl FRotation {
    pub fn new (_roll : f64, _pitch : f64, _yaw : f64,
    _x : f64, _y : f64, _z : f64, _w : f64) -> FRotation
    {
        FRotation { roll: _roll, pitch: _pitch, yaw: _yaw,
             x: _x, y: _y, z: _z, w: _w }
    }

    pub fn GetRoll(&mut self) -> &f64 {
        &self.roll
    }

    pub fn GetPitch(&mut self) -> &f64 {
        &self.pitch
    }

    pub fn GetYaw(&mut self) -> &f64 {
        &self.yaw
    }

    pub fn SetRoll(&mut self, _roll : f64) {
        self.roll = _roll;
    }

    pub fn SetPitch(&mut self, _pitch : f64) {
        self.pitch = _pitch
    }

    pub fn SetYaw(&mut self, _yaw : f64) {
        self.yaw = _yaw;
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

    pub fn GetW(&mut self) -> &f64 {
        &self.w
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

    pub fn SetW(&mut self, _w : f64) {
        self.w = _w;
    }

}

