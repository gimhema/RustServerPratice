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

    pub fn AddNewConnection(&mut self, id: i64, connToken: Token)
    {
        self.connections.insert(id, connToken);
    }

    pub fn GetConnetionByID(&mut self, id: i64) -> &Token
    {
        let mut ret = self.connections.get(&id);
        ret.clone().unwrap()
    }

    pub fn RemoveConnectionById(&mut self, id: i64)
    {
        self.connections.remove(&id);
    }
}

