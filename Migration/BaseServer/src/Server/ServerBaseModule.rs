use mio::event::Event;
use mio::event::Source;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::ops::DerefMut;
use std::str::from_utf8;
use std::{thread, time};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;
use mio::event;
use tokio::time::error::Elapsed;

use serde::{Serialize, Deserialize};

use super::GamePacketModule::GamePacket;
use super::ConnetionHandleModule::ConnectionHandler;

use super::MessageBufferModule::RecvMessageBuffer;
use super::MessageBufferModule::SendMessageBuffer;
use super::ServerFunctions::*;


use super::Server;

use std::ops::{Deref};

use crate::gGameLogic;
use crate::CallServerActionByFunctionHeader;
use crate::GameLogic::CharacterModule::Character;
use crate::GetGameLogic;
use crate::Server::GamePacketModule::SendGamePacket;
use crate::Server::GamePacketModule::SendGamePacketToConnect;
use crate::{gRecvMessageBuffer, gSendMessageBuffer, THREAD_SWITCH};
use crate::{GetThreadSwitch, SetThreadSwitch};

const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 1000;
const DATA: &[u8] = b"Hello Unreal Im Rust Server ! ! !\n";
const DATA2: &[u8] = b"Hi Unreal ! ! ! ! ! !\n";

const BUFFER_SIZE_LIMIT: usize = 10000;
const MAX_NUM_USER: i64 = 2;


// Private
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

pub fn update_logic(server: &mut ServerBase) {
    // Your update logic here
//    server.RecvMessageProcess();
//    server.UpdateProcess(); 이건 게임 로직이 별도로 실행해줌
}

pub struct ServerBase {
    clientHandler: ConnectionHandler,
    numUser: i64,
    step: i64
}

impl Deref for ServerBase {
    type Target = ConnectionHandler;

    fn deref(&self) -> &Self::Target {
        &self.clientHandler
    }
}

impl DerefMut for ServerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clientHandler
    }
}


impl ServerBase {

    //
    pub fn new() -> Self {
        let mut _clientHandler = ConnectionHandler::new();

        ServerBase {
            clientHandler: _clientHandler,
            numUser: 0,
            step: 0
        }
    }

    pub fn GetNumUser(&mut self) -> &i64
    {
        &self.numUser
    }

    pub fn IncreaseNumUser(&mut self)
    {
        GetGameLogic().write().unwrap().IncreaseUserNum();
    }

    pub fn AddNewPlayer(&mut self, _tcpStream : TcpStream, _token: Token)
    {
        let gLogic = gGameLogic.clone();
        let mut _newID = 0 as i64;
        // Spawn a new thread to perform a task
        let handle = thread::spawn(move || {
            // Acquire a write lock
            if let Ok(mut write_guard) = gLogic.write() {
                // Perform the task on the write-locked instance
                println!("Get Guard OK");
                _newID = write_guard.AddNewPlayer(_tcpStream, _token);
                // The write lock is automatically released when 'write_guard' goes out of scope
            }
        });

        handle.join().unwrap();

        let welcome_packet = GamePacket::new(
            -1,
            _newID,
            FunctionHeader::CONNECTION_SUCESSFUL.into(),
            vec![0.0],
            "Connection Check Message".to_string());
        
        self.SendGameMessage(Some(welcome_packet));
    }

    pub fn GetConnetionByToken(&mut self, token: Token) -> Option<&mut TcpStream>
    {
        self.clientHandler.GetConnetionByToken(token)
    }

    pub fn DecreaseNumUser(&mut self )
    {
        GetGameLogic().write().unwrap().DecreaseUserNum();        
    }

    pub fn RemovePlayerByID(&mut self, pid : i64)
    {
        GetGameLogic().write().unwrap().RemovePlayerByID(pid);
    }
        

    pub fn UpdateProcess(&mut self)
    {
        println!("Call Server Update");
    }

    // self.clientHandler.GetConnetionByToken(token)

    

    pub fn Start(&mut self) -> io::Result<()> 
    {
        env_logger::init();

        let mut userCount: i64 = 0;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let addr = "127.0.0.1:9000".parse().unwrap();
        let mut server = TcpListener::bind(addr)?;
    
        // Register the server with poll we can receive events for it.
        poll.registry().register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    
        // Map of `Token` -> `TcpStream`.
        // let mut connections = HashMap::new();

        let mut unique_token = Token(SERVER.0 + 1);


        loop {
            // println!("Set Poll Step : {}", self.step);
            poll.poll(&mut events, Some(Duration::from_millis(SERVER_TICK)))?;
    
            // println!("Iterate Event For Loop");
            for event in events.iter() {
                if event.token() == Token(0) && event.is_writable() {
                    println!("Writeable Event . . .");
                }
    
                match event.token() {
                    SERVER => loop {
                        // Received an event for the TCP server socket, which
                        // indicates we can accept an connection.
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) =>  (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // If we get a `WouldBlock` error we know our
                                // listener has no more incoming connections queued,
                                // so we can return to polling and wait for some
                                // more.
                                break;
                            }
                            Err(e) => {
                                // If it was any other kind of error, something went
                                // wrong and we terminate with an error.
                                return Err(e);
                            }
                        };
                        println!("Accepted connection from: {}", address);
    
                        let token = next(&mut unique_token);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;

                        println!("Add New Player");

                        let mut sendConnect = connection;
                        
                        self.AddNewPlayer(sendConnect, token);                        
                    

                        println!("SendGamePacket End");
                    },
                    token => {
                       let done = if let Some(connection)  = GetGameLogic().write().unwrap().GetUserConnectionsByToken(token) 
                        {
                            println!("Handle Connection Event");
                            handle_connection_event(poll.registry(), connection, event)?
                        } 
                        else 
                        {
                            // println!("User Disconnected . . 2 2");
                            // Sporadic events happen, we can safely ignore them.
                            false
                        };
    
                       if done {
                            //  GetGameLogic().write().unwrap()
                            // self.clientHandler.GetConnetionByToken(token)
                            println!("Disconn search . . .");
                            if let Some(mut connection)  = GetGameLogic().write().unwrap().GetUserConnectionsByToken(token)
                            {
                                println!("User Disconnected . . 1");
                                // poll.registry().deregister(connection);
                                // let removeID = self.clientHandler.GetIDByConnection(token);
                                // 두 과정은 하나의 함수로 표현해야함
                                // self.clientHandler.RemoveConnectionByToken(token);
                                // self.clientHandler.RemoveTokenPairByID(removeID);
                                // self.RemovePlayerByID(removeID);
                                // self.DecreaseNumUser();
                            }
                       }
                    }
                }
                // println!("For Loop End");
            }
            // println!("Calling update_logic");
            // update_logic(self);
            // println!("update_logic called");

            // println!("Set Poll End");

            self.step += 1;
    
            // 게임 로직에서 처리후 바로 Send하고있기때문에 필요없을수도있다.
            // if gSendMessageBuffer.GetNumElem() > 0 {
            //     while let Some(item) = gSendMessageBuffer.PopData() {
            //         let mut send_data = gSendMessageBuffer.PopData();
            //         let mut senderID = send_data.as_ref().unwrap().getSenderID();
            //         let mut destination = *send_data.as_ref().unwrap().getTargetID();
            //         // let _targetID = value.get

            //         let _targetToken = *self.clientHandler.GetTokenByID(destination).unwrap();
            //         let _connStream = self.clientHandler.GetConnetionByToken(_targetToken);

            //         if let send_msg = serde_json::to_string(&send_data)? {
            //             let serialized_msg = send_msg.as_bytes();
            //             // value.getTcpStream().write(serialized_msg);
            //             _connStream.unwrap().write(serialized_msg);
            //         }
            //     }
            // }
        }
    }


pub fn SendGameMessage(&mut self, message: Option<GamePacket>) {
        
    let send_data = match &message {
        Some(data) => {
            println!("Send Game Message Valid");
            data
        },
        None => {
            // Handle the case when packet is None
            println!("Send Game Message Exception 1");
            return;
        }
    };
        

    let destination = *send_data.getTargetID();

    let mut _target = Token(0);
    {
        _target = match GetGameLogic().write().unwrap().GetUserTokenByID(destination) {
            Some(token) => *token,
            None => {
                // Handle the case when GetTokenByID returns None
                println!("Find Token Failed : Invalid Player ID");
                return;
            }
        };
    }

    if let Ok(send_msg) = serde_json::to_string(&send_data) {
        let serialized_msg = send_msg.as_bytes();
        if let Some(_targetConn) = GetGameLogic().write().unwrap().GetUserConnectionsByToken(_target) {

            _targetConn.write(serialized_msg);
        }
    }

    println!("Send Game Message Call End");
}    




}


fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    println!("Handle Connection Event Start . . ");

    if event.is_readable() {
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        // We can (maybe) read from the connection.
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {

            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                println!("Received data: {}", str_buf.trim_end());
                // 받은 데이터 처리
                // 데이터를 수신전용 버퍼에 추가한다.
                let recvMsg = String::from(from_utf8(received_data).unwrap());

                CallServerActionByFunctionHeader(Some(recvMsg));

                
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }
    println!("Handle Connection Event End . . ");
    Ok(false)
}



fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}