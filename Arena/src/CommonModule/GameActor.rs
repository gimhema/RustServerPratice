

pub struct ActorStatus {
    health : i64,
    currentLocation : Vec<f64>,
    currentRotation : Vec<f64>
}

pub trait Actor {
    fn Create(&self);
    fn Update(&self);
}