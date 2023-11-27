
// My Custom Modules -- Rocky Linux Commit Test
mod ArenaServerModule;
mod GameLogicCore;
mod CommonModule;

use ArenaServerModule::{ArenaServerCoreModule};
use ArenaServerModule::{ArenaClientModule};
use ArenaServerModule::ArenaMessageModule::{ArenaMessage};
use ArenaServerModule::ArenaClientModule::{ArenaPlayer};
use ArenaServerModule::ArenaServerActionMappingModule::{ServerActionMappingInitialize};

// You can run this example from the root of the mio repo:
// cargo run --example tcp_server --features="os-poll net"
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



extern crate lazy_static;
use lazy_static::lazy_static;

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

// Some data we'll send over the connection.
// 언리얼 클라이언트에게 아래와 같은 메세지를 보낼것이다.
const DATA: &[u8] = b"Hello Unreal Im Rust Server ! ! !\n";
const DATA2: &[u8] = b"Hi Unreal ! ! ! ! ! !\n";
const DATA3: &[u8] = b"Hi Unreal This Message Update\n";
type ArenaEventAction = fn(String) -> i64;

static gUserIndex: i64 = 0;
static sendMessageBuffer: Mutex<VecDeque<ArenaMessage>> = Mutex::new(VecDeque::new());
const RECV_LIMIT: usize = 10000;
const SERVER_TICK: u64 = 500;
static recvMessageBuffer: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

lazy_static!{
    // 생각해보니까 굳이 문자열일 필요는 없는것같다...
    static ref serverActionMap: Mutex<HashMap<i64, ArenaEventAction>> = Mutex::new(HashMap::new());

    // 토큰을 키로 괸리하는 유저 관리 컨테이너
    // 이러면 전반적인 구조 수정이 좀 필요하다..
    static ref gUserContainer: Mutex<HashMap<Token, ArenaPlayer>> = Mutex::new(HashMap::new());
}


#[cfg(not(target_os = "wasi"))]
fn main() -> io::Result<()> {

    // test

    use mio::event;
    use tokio::time::error::Elapsed;

    use crate::ArenaServerModule::ArenaGameCommonModule::InstanceGame;

    // Server Network Setting
    env_logger::init();

    // Initialize Message Function . . . 
    ServerActionMappingInitialize();

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


//    ArenaServerCoreModule::ArenaServerInitialize(); // Server Initialize 
//    let mut instanceGame =  InstanceGame::new(0, 0, 2);
    let instanceGame = Arc::new(Mutex::new(InstanceGame::new(0, 0, 2)));

    // Clone Arc for each thread
    let instance_game_action = Arc::clone(&instanceGame);

    // Spawn threads using the cloned Arcs
    let instanceGameUpdateLogic = thread::spawn(move || {
        println!("Spawned Wait Thead");
        let mut instance_game = instance_game_action.lock().unwrap();
        instance_game.GameLogicUpdate();
    });


//    let instance_recvMsg_action = Arc::clone(&instanceGame);
    let instanceRecvMsgLogic = thread::spawn(move || {
        println!("Spawned RecvMessage Thead");
        RecvMessageProcessLoop();
    });



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
                    connections.insert(token, sendConnect);
                    ArenaClientModule::AddNewUserToContainer(userCount, token, "test".to_string());                     
                  
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
            let mut send_data_buffer = sendMessageBuffer.lock().unwrap();
            
            // 메세지 버퍼가 비어있지 않다면
            if send_data_buffer.capacity() > 0 {
                if let Some(send_data) = send_data_buffer.pop_back() {
                    let destination = send_data.get_header();
                    let send_msg = send_data.get_msg().as_bytes();
                   
                   // message의 토큰을 보고
                   // 같은 토큰인 경우에만 메세지를 보낸다.
                   // 어떤 토큰에 보낼것인가? << 즉 모두에게 보내야하는지, 특정유저에게만 보내야하는지는 송신전처리에서 봐야한다.
                    if key == destination {
                        value.write(send_msg);
                    }
                }
            }
        }
    }

    instanceGameUpdateLogic.join().unwrap();
    instanceRecvMsgLogic.join().unwrap();

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
                // 받은 데이터 처리
                // 데이터를 수신전용 버퍼에 추가한다.
                let recvMsg = String::from(from_utf8(received_data).unwrap());        
                if(recvMessageBuffer.lock().unwrap().capacity() < RECV_LIMIT)
                {
                    recvMessageBuffer.lock().unwrap().push_back(recvMsg);                
                }
                
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

pub fn RecvMessageProcessLoop() {
    let ten_millis = time::Duration::from_millis(1000);
    loop
    {
//        if (self.IsConclude()) { break; }
        
        thread::sleep(ten_millis);
        println!("Loop Recv Message . . .");
//        if( true == self.IsStart() )
//        {
//            if (self.isGameConclusion == true) { break; } // 게임이 끝났다면 종료
//            // recvMessageBuffer.lock().unwrap() 에서 메세지를 꺼낸다. (main.rs 275~290 line)
//            let msg = recvMessageBuffer.lock().unwrap().pop_back();
//            // "uid:mid:mVal" 형식으로 받아올것이다.
//            let mut data = ArenaMessageData::CreateByMessage(msg.unwrap());
//            // 메세지를 꺼내서 이리저리 뜯어본다.
//            let mut uid = data.get_uid(); // User ID
//            let mut mid = data.get_mid(); // Message ID
//            let mut mVal = data.get_value(); // Message Function
//    
//            // 콜백함수에는 헤더 이후에 작성된 정보들이 저장되어있다.
//            self.CallServerAction(uid, mid, mVal);
//        }
    }
}



#[cfg(target_os = "wasi")]
fn main() {
    panic!("can't bind to an address with wasi")
}




