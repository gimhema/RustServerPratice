use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use std::fmt::Debug;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Person {
    name : String,
    age : i64,
    birthday : i64
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connected Client");
    let config = config::standard();

    loop {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Client disconnected");
            return Ok(());
        }

        // println!("Message from Client: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
        let (decoded_person, len_person) : (Entity, usize) = bincode::decode_from_slice(&buffer, config).unwrap();
        println!("Decoded Person : {:?}", decoded_person);
        println!("Bytes Consumed: {}", len_person);

        let entity: Entity = Entity{x : 1.2, y : 3.3};
        let encoded_entity = bincode::encode_to_vec(&entity, config).unwrap();
        let test_packet: &[u8] = &encoded_entity;



        stream.write_all(test_packet)?;
        // stream.write_all(b"Server Response")?;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream)?;
    }

    Ok(())
}

/*
    let config = config::standard();
 
    let entity = Entity{x : 1.2, y : 3.3};

    let encoded_entity = bincode::encode_to_vec(&entity, config).unwrap();

    let (decoded_entity, len) : (Entity, usize) = bincode::decode_from_slice(&encoded_entity, config).unwrap();

    let person_config = config::standard();

    let person = Person{ name : "Mkie".to_string(), age : 10, birthday : 20240313 };

    let encoded_person = bincode::encode_to_vec(&person, person_config).unwrap();

    println!("Encoded Person Bytes: {:?}", encoded_entity);

    let (decoded_person, len_person) : (Person, usize) = bincode::decode_from_slice(&encoded_person, person_config).unwrap();

    println!("Decoded Person : {:?}", decoded_person);
    println!("Bytes Consumed: {}", len_person);
*/

