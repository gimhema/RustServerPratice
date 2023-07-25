


pub trait Manager {
    fn Initialize(&self);
    fn Update(&self);
}

pub trait AutoActionManager {
    fn AutoUpdate(&self);
}