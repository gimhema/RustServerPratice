use bincode::{config, Decode, Encode};


// Struct -> Binary Packet
#[macro_export]
macro_rules! encode_packet {
    ($packet:expr, $config:expr) => {
        {
            bincode::encode_to_vec(&$packet, $config).unwrap()
        }
    };
}


// Binary Packet -> Struct
#[macro_export]
macro_rules! decode_packet {
    ($encoded:expr, $Type: expr ,$config:expr) => {
        {
            let (decoded, len): (Type, usize) = bincode::decode_from_slice(&$encoded[..], $config).unwrap();
            (decoded, len)
        }
    };
}

