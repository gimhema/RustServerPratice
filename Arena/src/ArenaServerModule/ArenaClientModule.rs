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
    userName: String
}

pub struct ArenaClientNetworkInfo {
    userToken: Token,
    userConnectStream: TcpStream
}

impl  ArenaClient {
    pub fn Create(&mut self, ID: i64, Password: String, userName: String, IP: String) {
        self.userID = ID;
        self.userPW = Password;
        self.userName = userName;
    }
    pub fn Connect() {

    }
}

pub struct ArenaClientManager {
    clientContainer: HashMap<i64, ArenaClient>, // ID, Client Information
    clientNetworkContainer: HashMap<i64, ArenaClientNetworkInfo> // ID, Arena Network Information
}

impl ArenaClientManager {
    pub fn Initialize(&mut self)
    {
        println!("Arena Client Manager Initialize");
        
    }

    pub fn new() -> ArenaClientManager {
        ArenaClientManager { 
            clientContainer: HashMap::new(),
            clientNetworkContainer: HashMap::new()
        }
    }

    pub fn AddNewUserToContainer(&mut self, id: i64, newClient: ArenaClient)
    {
        self.clientContainer.insert(id, newClient);
    }
    pub fn AddNetUserConnetion(&mut self, id: i64, connection: TcpStream, token: &Token)
    {
        let _connectionInfoRef = ArenaClientNetworkInfo{
            userToken: *token,
            userConnectStream: connection
        };

        self.clientNetworkContainer.insert(id, _connectionInfoRef);
    }

    pub fn GetUserInformationByID(&mut self, id: i64) -> Option<&ArenaClient>
    {
        self.clientContainer.get(&id)
    }

    pub fn GetUserConnectionByID(&mut self, id: i64) -> Option<&ArenaClientNetworkInfo>
    {
        self.clientNetworkContainer.get(&id)
    }

    pub fn GetUserConnectStreamByID(&mut self, id: i64) -> &TcpStream
    {
        &self.clientNetworkContainer.get(&id).unwrap().userConnectStream
    }

}

