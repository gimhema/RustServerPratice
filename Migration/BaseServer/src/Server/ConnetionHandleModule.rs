use std::io::Write;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::Token;

pub struct Connection
{
    id: i64,
    tcpStream: TcpStream
}

impl Connection
{
    pub fn new(_id: i64, _connStream: TcpStream) -> Self {
        Connection { id: _id, tcpStream: _connStream }
    }

    pub fn getID(&self) -> &i64 {
        &self.id
    }

    pub fn getTcpStream(&self) -> &TcpStream {
        &self.tcpStream
    }
}

pub struct ConnectionHandler 
{
//    connections: HashMap<i64, Token>
    connections: HashMap<Token, Connection>
}

impl ConnectionHandler
{
    pub fn new() -> Self {
        let mut _connetions = HashMap::new();
        ConnectionHandler{
            connections : _connetions
        }
    }

    pub fn GetConnections(&mut self) -> &HashMap<Token, Connection>
    {
        &self.connections
    }

    pub fn AddNewConnection(&mut self, id: i64, _tcpStream: TcpStream, _token: Token)
    {
        let mut conn = Connection::new(id, _tcpStream);
        self.connections.insert(_token, conn);
    }

    pub fn GetConnetionByToken(&mut self, token: Token) -> Option<&mut TcpStream>
    {
        if let Some(connection) = self.connections.get_mut(&token) {
            Some(&mut connection.tcpStream)
        } else {
            None
        }
    }

    pub fn RemoveConnectionByToken(&mut self, token: Token)
    {
        self.connections.remove(&token);
    }
}

