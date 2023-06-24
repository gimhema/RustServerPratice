use std::collections::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::{Token};

pub fn Initailize() {
    println!("Arena Client Initialize");
}

pub struct ConnectInfo {
    connectToken: Token,
    connectStream: TcpStream
}

impl ConnectInfo {
    pub fn Create(&mut self, token: Token, stream: TcpStream) 
    {
        self.connectToken = token;
    }
}


pub struct ArenaClient {
    userID: i64,
    userPW: String,
    userName: String,
    connectedIPAddress: String,
    connectToken: Token,
    connectStream: TcpStream
}

impl  ArenaClient {
    pub fn Create(&mut self, ID: i64, Password: String, userName: String, IP: String, token : Token, stream: TcpStream) {
        self.userID = ID;
        self.userPW = Password;
        self.userName = userName;
        self.connectedIPAddress = IP;
        self.connectToken = token;
        self.connectStream = stream;
    }
    pub fn Connect() {

    }
}

pub struct ArenaClientManager {
    clientContainer: HashMap<Token, ArenaClient>
}

impl ArenaClientManager {
    pub fn Initialize(&mut self)
    {
        println!("Arena Client Manager Initialize");
        
        self.clientContainer = HashMap::new();
    }

    pub fn AddNewUserToContainer(&mut self, connectToken: Token, newClient: ArenaClient)
    {
        self.clientContainer.insert(connectToken, newClient);
    }
}

