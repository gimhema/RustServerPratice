use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashMap;
use mio::Token;

pub struct ConnectionHandler 
{
    connections: HashMap<i64, Token>
}

impl ConnectionHandler
{
    pub fn new() -> Self {
        let mut _connetions = HashMap::new();
        ConnectionHandler{
            connections : _connetions
        }
    }
}

