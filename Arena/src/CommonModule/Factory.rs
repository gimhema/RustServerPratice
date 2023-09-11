


pub trait Factory {
    fn Initialize(&self);
    fn CreateUnit(&self, unitKind: i64);
}