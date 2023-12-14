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

const SERVER: Token = Token(0);

pub struct ServerBase {

}

impl ServerBase {

    fn Initialize(&mut self)
    {

    }

    fn Start(&mut self)
    {
        env_logger::init();

        let mut userCount: i64 = 0;
        let mut poll = Poll::new();

        let addr = "127.0.0.1:9000".parse().unwrap();
        let mut server = TcpListener::bind(addr);
    
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
                        connections.insert(token, sendConnect);
                        // ArenaClientModule::AddNewUserToContainer(userCount, token, "test".to_string());                     
                      
                    },
                    token => {
                        
    
                       let done = if let Some(connection) = connections.get_mut(&token) {
                           // handle_connection_event(poll.registry(), connection, event)?
                       } else {
                           // Sporadic events happen, we can safely ignore them.
                           // false
                       };
    
                       if done {
                           if let Some(mut connection) = connections.remove(&token) {
                               // poll.registry().deregister(&mut connection)?;
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
                // let mut send_data_buffer = sendMessageBuffer.lock().unwrap();
                
                // 메세지 버퍼가 비어있지 않다면
                //if send_data_buffer.capacity() > 0 {
                //    if let Some(send_data) = send_data_buffer.pop_back() {
                //        let destination = send_data.get_header();
                //        let send_msg = send_data.get_msg().as_bytes();
                //       
                //       // message의 토큰을 보고
                //       // 같은 토큰인 경우에만 메세지를 보낸다.
                //       // 어떤 토큰에 보낼것인가? << 즉 모두에게 보내야하는지, 특정유저에게만 보내야하는지는 송신전처리에서 봐야한다.
                //        if key == destination {
                //            value.write(send_msg);
                //        }
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
