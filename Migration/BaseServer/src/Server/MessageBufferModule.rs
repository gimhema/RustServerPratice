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


