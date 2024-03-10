// GamePacket as Byte (Not Json)
use std::mem;
use std::time::Instant;

const DATA_SIZE: usize = 1400; // 1MB 데이터 크기로 가정


macro_rules! packet_build {
    ($packet:expr, $T:ty) => {{
        let start_serialization = Instant::now();
        let buffer: [u8; mem::size_of::<$T>()] =
            unsafe { mem::transmute_copy(&$packet) };
        let end_serialization = Instant::now();

        // 여기서부터는 매크로가 생성한 코드 외의 추가 코드를 추가할 수 있습니다.

        // 예: println!("Serialization time: {:?}", end_serialization - start_serialization);

        buffer
    }};
}

macro_rules! packet_unpack {
    ($data:expr) => {
        
    };
}
