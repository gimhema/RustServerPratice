use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let config = config::standard();

    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = bincode::encode_to_vec(&world, config).unwrap();

    // The length of the vector is encoded as a varint u64, which in this case gets collapsed to a single byte
    // See the documentation on varint for more info for that.
    // The 4 floats are encoded in 4 bytes each.
    assert_eq!(encoded.len(), 1 + 4 * 4);

    println!("Encoded Bytes: {:?}", encoded);

    let (decoded, len): (World, usize) = bincode::decode_from_slice(&encoded[..], config).unwrap();

    println!("Decoded World: {:?}", decoded);
    println!("Bytes Consumed: {}", len);
}


/*

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let config = config::standard();
 
    let entity = Entity{x : 1.2, y : 3.3};
 //   let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

//    let encoded: Vec<u8> = bincode::encode_to_vec(&world, config).unwrap();

    let encoded_entity = bincode::encode_to_vec(&entity, config).unwrap();

    // The length of the vector is encoded as a varint u64, which in this case gets collapsed to a single byte
    // See the documentation on varint for more info for that.
    // The 4 floats are encoded in 4 bytes each.
//    assert_eq!(encoded.len(), 1 + 4 * 4);
//   assert_eq!(encoded_entity.len(), 1 + 4 * 4);

 //   println!("Encoded Bytes: {:?}", encoded);
    println!("Encoded Entity Bytes: {:?}", encoded_entity);

//    let (decoded, len): (World, usize) = bincode::decode_from_slice(&encoded[..], config).unwrap();

    let (decoded_entity, len) : (Entity, usize) = bincode::decode_from_slice(&encoded_entity, config).unwrap();

//    println!("Decoded World: {:?}", decoded);
    println!("Decoded Entity : {:?}", decoded_entity);
    println!("Bytes Consumed: {}", len);
}

*/
