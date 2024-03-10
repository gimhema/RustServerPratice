use rand::Rng;
use std::mem;
use std::time::Instant;

const DATA_SIZE: usize = 1400; // 1MB 데이터 크기로 가정

// 패킷 정의
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct GamePacket {
    opcode: u16,
    player_id: u32,
    pos_x: f32,
    pos_y: f32,
}

fn main() {
    // 서버에서 클라이언트로 보낼 패킷 생성
    let packet_to_send = GamePacket {
        opcode: 1001,
        player_id: 123,
        pos_x: 10.5,
        pos_y: 20.0,
    };

    // 패킷을 바이트 배열로 변환 및 시간 측정
    let start_serialization = Instant::now();
    let buffer: [u8; mem::size_of::<GamePacket>()] =
        unsafe { mem::transmute_copy(&packet_to_send) };
    let end_serialization = Instant::now();

    // 네트워크를 통해 패킷 전송 (여기서는 간단하게 화면에 출력으로 대체)
    let start_deserialization = Instant::now();
    let received_packet: GamePacket =
        unsafe { mem::transmute_copy(&buffer) };
    let end_deserialization = Instant::now();

    // 전송된 실제 데이터의 크기
    let actual_data_size = buffer.len();

    println!("Packet Sent:");
    println!("Opcode: {}", packet_to_send.opcode);
    println!("Player ID: {}", packet_to_send.player_id);
    println!("Position: ({}, {})", packet_to_send.pos_x, packet_to_send.pos_y);
    println!("Actual Data Size: {} bytes", actual_data_size);
    println!(
        "Serialization Time: {} seconds",
        (end_serialization - start_serialization).as_secs_f64()
    );

    // 클라이언트에서 서버로부터 수신한 패킷 처리
    println!("\nPacket Received:");
    println!("Opcode: {}", received_packet.opcode);
    println!("Player ID: {}", received_packet.player_id);
    println!("Position: ({}, {})", received_packet.pos_x, received_packet.pos_y);
    println!("Actual Data Size: {} bytes", actual_data_size);
    println!(
        "Deserialization Time: {} seconds",
        (end_deserialization - start_deserialization).as_secs_f64()
    );
}