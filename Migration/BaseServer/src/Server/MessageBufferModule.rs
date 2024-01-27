use mio::event::Event;
use mio::event::Source;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::borrow::BorrowMut;
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



pub struct RecvMessageBuffer {
    container: Mutex<VecDeque<String>>
}

pub struct SendMessageBuffer {
    container: Mutex<VecDeque<GamePacket>>
}

impl RecvMessageBuffer {
    pub fn new() -> Self {
        let mut _recvMessageBuffer = Mutex::new(VecDeque::new());
        RecvMessageBuffer { container: _recvMessageBuffer }
    }

    pub fn GetContainer(&mut self) -> &Mutex<VecDeque<String>>
    {
        &self.container
    }

    pub fn PushBackData(&self, recvMsg: String)
    {
        self.container.lock().unwrap().push_back(recvMsg);
    }

    pub fn PopData(&self) -> Option<String>
    {
        self.container.lock().unwrap().pop_back()
    }

    pub fn GetNumElem(&self) -> usize
    {
        self.container.lock().unwrap().len()
    }

    pub fn IsEmpty(&self) -> bool
    {
        self.container.lock().unwrap().is_empty()
    }
    
}

impl SendMessageBuffer {
    pub fn new() -> Self {
        let mut _sendMessageBuffer = Mutex::new(VecDeque::new());
        SendMessageBuffer { container: _sendMessageBuffer }
    }

    pub fn GetContainer(&mut self) -> &Mutex<VecDeque<GamePacket>>
    {
        &self.container
    }

    pub fn PushBackData(&self, sendMsg: GamePacket)
    {
        self.container.lock().unwrap().push_back(sendMsg);
    }

    pub fn PopData(&self) -> Option<GamePacket>
    {
        self.container.lock().unwrap().pop_back()
    }

    pub fn GetNumElem(&self) -> usize
    {
        self.container.lock().unwrap().len()
    }

    pub fn IsEmpty(&self) -> bool
    {
        self.container.lock().unwrap().is_empty()
    }
}