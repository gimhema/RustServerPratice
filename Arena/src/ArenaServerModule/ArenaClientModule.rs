use std::collections::HashMap;
use std::io::Write;
use mio::net::{TcpListener, TcpStream};
use mio::{Token};

const SENDTEST: &[u8] = b"Hi Unreal This Message Is Update Loop\n";

pub fn SendTestFunction(connectionStream: &mut TcpStream)
{
    connectionStream.write(SENDTEST);
}

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
    userConnectStream: TcpStream,
    userIdentify: i64
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
    clientNetworkContainer: HashMap<Token, ArenaClientNetworkInfo>, // ID, Arena Network Information
    clientTokenVec: Vec<Token>
}

impl ArenaClientManager {
    pub fn Initialize(&mut self)
    {
        println!("Arena Client Manager Initialize");
        
    }

    pub fn new() -> ArenaClientManager {
        ArenaClientManager { 
            clientContainer: HashMap::new(),
            clientNetworkContainer: HashMap::new(),
            clientTokenVec: Vec::new()
        }
    }

    pub fn AddNewTokenToVec(&mut self, token: Token)
    {
        self.clientTokenVec.push(token);
    }

    pub fn AddNewUserToContainer(&mut self, id: i64, newClient: ArenaClient)
    {
        self.clientContainer.insert(id, newClient);
    }
    pub fn AddNetUserConnetion(&mut self, id: i64, connection: TcpStream, token: &Token)
    {
        let _connectionInfoRef = ArenaClientNetworkInfo{
            userToken: *token,
            userConnectStream: connection,
            userIdentify: id
        };

        self.clientNetworkContainer.insert(*token, _connectionInfoRef);
    }

    pub fn GetUserInformationByID(&mut self, id: i64) -> Option<&ArenaClient>
    {
        self.clientContainer.get(&id)
    }

    pub fn GetUserConnectionByToken(&mut self, token: &Token) -> Option<&ArenaClientNetworkInfo>
    {
        self.clientNetworkContainer.get(token)
    }

    pub fn GetUserConnectStreamByToken(&mut self, token: &Token) -> &mut TcpStream
    {
        &mut self.clientNetworkContainer.get_mut(token).unwrap().userConnectStream
    }

    pub fn RemoveConnectionUseToken(&mut self, token: &Token)
    {
        self.clientNetworkContainer.remove(token);
    }

    pub fn CheckValidConnection(&mut self, token: &Token) -> bool
    {
        let mut result = false;
        if let Some(_conn) = self.clientNetworkContainer.get_mut(token){
            result = true;
        }else {
            result = false;
        }
        result
    }

    pub fn SendTestMessage(&mut self, token: Token)
    {
        self.GetUserConnectStreamByToken(&token).write(SENDTEST);
    }

    pub fn SendUpdateLoop(&mut self)
    {
        loop {
            for (key, value) in &self.clientNetworkContainer{
                let _token = key.clone();
 //               SendTestFunction(value.userConnectStream);
            }
        }
    }

    pub fn GetClientNetworkContainer(&mut self) -> &HashMap<Token, ArenaClientNetworkInfo>
    {
        &self.clientNetworkContainer
    }

}

