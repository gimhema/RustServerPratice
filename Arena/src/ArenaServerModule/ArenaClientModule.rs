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
        
    }

    pub fn new() -> ArenaClientManager {
        ArenaClientManager { clientContainer: HashMap::new() }
    }

    pub fn MakeNewClient(&mut self, 
        ID: i64, Password: String, 
        userName: String, 
        IP: String, token : Token, stream: TcpStream) -> ArenaClient {
            ArenaClient { 
                userID: ID, userPW: Password, 
                userName: userName, connectedIPAddress: IP, 
                connectToken: token, connectStream: stream 
            }
    }

    pub fn AddNewUserToContainer(&mut self, connectToken: Token, newClient: ArenaClient)
    {
        self.clientContainer.insert(connectToken, newClient);
    }

    // pub fn GetUserInfoByToken(&mut self, token: Token) -> ArenaClient {
    //     let result = self.clientContainer.get(&token);
        
    //     let _ID = result.unwrap().userID;
    //     let _PW = &result.unwrap().userPW;
    //     let _Name = &result.unwrap().userName;
    //     let _IP = &result.unwrap().connectedIPAddress;
    //     let _token = result.unwrap().connectToken;
    //     let mut _ConnectionStream = &result.unwrap().connectStream;

    //     ArenaClient { 
    //         userID: _ID, userPW: _PW.to_string()
    //         , userName: _Name.to_string(), connectedIPAddress: _IP.to_string()
    //         , connectToken: _token, connectStream: _ConnectionStream }
    // }

}

