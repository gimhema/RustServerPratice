use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::Token;

pub struct Connection
{
    token: Token,
    tcpStream: TcpStream
}

impl Connection
{
    pub fn new(_token: Token, _connStream: TcpStream) -> Self {
        Connection { token: _token, tcpStream: _connStream }
    }
}

pub struct ConnectionHandler 
{
//    connections: HashMap<i64, Token>
    connections: HashMap<i64, Connection>
}

impl ConnectionHandler
{
    pub fn new() -> Self {
        let mut _connetions = HashMap::new();
        ConnectionHandler{
            connections : _connetions
        }
    }

    pub fn AddNewConnection(&mut self, id: i64, connection: Connection)
    {
        self.connections.insert(id, connection);
    }

    pub fn GetConnetionByID(&mut self, id: i64) -> &Connection
    {
        let mut ret = self.connections.get(&id);
        ret.clone().unwrap()
    }

    pub fn RemoveConnectionById(&mut self, id: i64)
    {
        self.connections.remove(&id);
    }
}

