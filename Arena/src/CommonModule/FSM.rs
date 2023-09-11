use std::collections::HashSet;

pub enum FSMStatusType {
    NONE,
    WAIT,
    MOVE,
    HOLD,
    ATTACK,
    DEAD,
}

pub struct FSMStatus {
    current : FSMStatusType,
    stateSet : HashSet<FSMStatusType>
}

pub trait FSMBehavior {
    fn Initailize(&self);
    fn ChangeState(&self, _state : FSMStatusType) -> FSMStatusType;
    fn Wait(&self);
    fn Move(&self);
    fn Hold(&self);
    fn Attack(&self);
    fn Dead(&self);
}