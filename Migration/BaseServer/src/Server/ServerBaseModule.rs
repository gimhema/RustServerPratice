use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;
use mio::event;
use tokio::time::error::Elapsed;

use super::GamePacketModule::GamePacket;
use super::ConnetionHandleModule::ConnectionHandler;


const SERVER: Token = Token(0);
const SERVER_TICK: u64 = 500;
const DATA: &[u8] = b"Hello Unreal Im Rust Server ! ! !\n";
const DATA2: &[u8] = b"Hi Unreal ! ! ! ! ! !\n";

// Private
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

pub struct ServerBase {
    recvMessageBuffer: Mutex<VecDeque<String>>,
    sendMessageBuffer: Mutex<VecDeque<GamePacket>>,
    clientHandler: ConnectionHandler
}

impl ServerBase {

    //
    pub fn new() -> Self {
        let mut _recvMessageBuffer = Mutex::new(VecDeque::new());
        let mut _sendMessageBuffer = Mutex::new(VecDeque::new());
        let mut _clientHandler = ConnectionHandler::new();

        ServerBase {
            recvMessageBuffer : _recvMessageBuffer,
            sendMessageBuffer : _sendMessageBuffer,
            clientHandler: _clientHandler
        }
    }


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
        let mut connections = HashMap::new();

        let mut unique_token = Token(SERVER.0 + 1);

        loop {
            println!("Set Poll");
            poll.poll(&mut events, Some(Duration::from_millis(SERVER_TICK)))?;
    
            println!("Iterate Event For Loop");
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
    
                        let mut sendConnect = connection;
                        sendConnect.write(DATA2);
    
                        userCount += 1;
    
                        // 유저의 카운트 수를 보고 컷을 해야한다.
//                        self.clientHandler.AddNewConnection(0,token);
                        connections.insert(token, sendConnect);
                      
                    },
                    token => {
                        
    
                       let done = if let Some(connection) = connections.get_mut(&token) {
                           handle_connection_event(poll.registry(), connection, event)?
                       } else {
                           // Sporadic events happen, we can safely ignore them.
                           false
                       };
    
                       if done {
                           if let Some(mut connection) = connections.remove(&token) {
                               poll.registry().deregister(&mut connection)?;
                           }
                       }
                    }
                }
                println!("For Loop End");
            }
            println!("Set Poll End");
    
            // sendBuffer에 저장되어 있는 메세지를 확인하고 있을때마다 
            // 정해진 header로 메세지를 보낸다
            // 메세지용 클래스도 하나 필요하겠네..
            for (key, value) in &mut connections {
                let mut send_data_buffer = self.sendMessageBuffer.lock().unwrap();
                
                // 메세지 버퍼가 비어있지 않다면
                if send_data_buffer.capacity() > 0 {
                    if let Some(send_data) = send_data_buffer.pop_back() {
                        
                        // let mut data = send_data.getSenderID()
                        
                        // let destination = send_data.get_header();
                        // let mut send_msg = &self.GamePacketSerialize(&send_data).unwrap().as_bytes();
                       
                       // message의 토큰을 보고
                       // 같은 토큰인 경우에만 메세지를 보낸다.
                       // 어떤 토큰에 보낼것인가? << 즉 모두에게 보내야하는지, 특정유저에게만 보내야하는지는 송신전처리에서 봐야한다.
                       // if key == destination {
//                            value.write(send_msg.unwrap().as_bytes_mut());
                       // }
                    }
                }
            }
        }
    }

    fn Update(&mut self)
    {

    }

    fn RecvLoop(&mut self)
    {

    }

    fn SendLoop(&mut self)
    {

    }

}



fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    println!("Handle Connection Event Start . . ");

    if event.is_writable() {
        // We can (maybe) write to the connection.
        match connection.write(DATA) {
            // We want to write the entire `DATA` buffer in a single go. If we
            // write less we'll return a short write error (same as
            // `io::Write::write_all` does).
            Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => {
                // After we've written something we'll reregister the connection
                // to only respond to readable events.
                registry.reregister(connection, event.token(), Interest::READABLE)?
            }
            // Would block "errors" are the OS's way of saying that the
            // connection is not actually ready to perform this I/O operation.
            Err(ref err) if would_block(err) => {}
            // Got interrupted (how rude!), we'll try again.
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, connection, event)
            }
            // Other errors we'll consider fatal.
            Err(err) => return Err(err),
        }
    }

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
                // if(recvMessageBuffer.lock().unwrap().capacity() < RECV_LIMIT)
                // {
                //     recvMessageBuffer.lock().unwrap().push_back(recvMsg);                
                // }
                
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
