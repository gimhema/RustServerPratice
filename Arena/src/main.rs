// My Custom Modules
mod ArenaServerModule;
mod GameLogicCore;

use ArenaServerModule::{ArenaServerCoreModule};
use ArenaServerModule::{ArenaClientModule};



// You can run this example from the root of the mio repo:
// cargo run --example tcp_server --features="os-poll net"
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Mutex;

extern crate lazy_static;
use lazy_static::lazy_static;

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

// Some data we'll send over the connection.
// 언리얼 클라이언트에게 아래와 같은 메세지를 보낼것이다.
const DATA: &[u8] = b"Hello Unreal Im Rust Server ! ! !\n";
const DATA2: &[u8] = b"Hi Unreal ! ! ! ! ! !\n";
const DATA3: &[u8] = b"Hi Unreal This Message Update\n";


// lazy_static!{
//     static ref sendMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
//     static ref recvMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());    
// }
static sendMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
static recvMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

// let mut sendMessageBuffer: VecDeque<String> = VecDeque::new();
// let mut recvMessageBuffer: VecDeque<String> = VecDeque::new();

// static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

#[cfg(not(target_os = "wasi"))]
fn main() -> io::Result<()> {

    // test

    use mio::event;
    use tokio::time::error::Elapsed;

//    use crate::ArenaServerModule::ArenaClientModule::ArenaClient;
    use crate::ArenaServerModule::ArenaClientModule::ArenaClientNetworkInfo;

    env_logger::init();

    let mut userCount: i64 = 0;

    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // Register the server with poll we can receive events for it.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;

    // Map of `Token` -> `TcpStream`.
    let mut connections = HashMap::new();


    // Unique token for each incoming connection.
    let mut unique_token = Token(SERVER.0 + 1);

    println!("You can connect to the server using `nc`:");
    println!(" $ nc 127.0.0.1 9000");
    println!("You'll see our welcome message and anything you type will be printed here.");


    loop {
        println!("Set Poll");
        poll.poll(&mut events, Some(Duration::from_millis(500)))?;

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

                    // 함수화 테스트
                    println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;

                    let mut sendConnect = connection;
                    sendConnect.write(DATA2);
                    // token, connetcion

//                    let mut _tempConnection = cltManager.GetUserConnectStreamByID(userCount);
//                    AddClientToContainer(&mut connections, sendConnect, &token);
                    userCount += 1;

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
        for (key, value) in &mut connections{
           value.write(DATA3);
        }
    }

//    sendHandle.join().unwrap();
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

/// Returns `true` if the connection is done.
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
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
            
            // 여기서부터 내가 수정한 부분
            // 데이터를 받았을 때 다시 전송할 수 있도록 추가만 해두었다.
            // event.is_writable() 아래 부분의 소스를 복붙했다.
            match connection.write(DATA) {
                Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
                Ok(_) => {

                    registry.reregister(connection, event.token(), Interest::READABLE)?
                }
                Err(ref err) if would_block(err) => {}
                Err(ref err) if interrupted(err) => {
                    return handle_connection_event(registry, connection, event)
                }
                Err(err) => return Err(err),
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

#[cfg(target_os = "wasi")]
fn main() {
    panic!("can't bind to an address with wasi")
}

fn AddClientToContainer(connections: &mut HashMap<Token, TcpStream>, connection: TcpStream, token: &Token)
{
    connections.insert(*token, connection); 
}

fn SendMessageAll(connections: &mut HashMap<Token, TcpStream>)
{
    // Send the message to all
    // 접속중인 모든 유저에게 메세지를 전송합니다
}

fn SendMessageToTarget()
{
    // Send the message to user as given paramter
    // 정해진 유저에게 메세지를 전송합니다.
}



