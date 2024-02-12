use std::io::{self, Write};
use tokio::io::{self as other_io, AsyncWriteExt};
use tokio::net::TcpStream;
use super::ClientCommandModule::*;

const SERVER_IP: &str = "127.0.0.1:8080";

pub struct PromptClientMain {
    id: i64,
    isConnect: bool,
    connectStream: Option<TcpStream>,
}

impl PromptClientMain {
    
    pub fn new(_id: i64) -> Self {
        // let mut stream = TcpStream::connect(SERVER_IP).await.unwrap();
        PromptClientMain{
            id: _id,
            isConnect: false,
            connectStream: None
        }
    }

    fn get_stream(&self) -> Option<&TcpStream> {
        self.connectStream.as_ref()
    }

    pub fn Start(&mut self) {
        // Connection 초기화 및 서버와 연결 시도
        println!("Prompt Client Start id : {}", self.id);

        self.isConnect = true; // 테스트

        // 서버와 연결이 잘 됐다면
        if (true == self.isConnect)
        {
            println!("Prompt Client Running . . . .");
            self.Run();
        }
        else 
        {
            // 대기중
            self.Wait();            
        }
    }

    async fn set_stream(&mut self, address: &str) -> io::Result<()> {
        if let Some(_) = &self.connectStream {
            // 이미 설정된 경우 에러 처리 또는 덮어쓰기 등의 로직을 추가할 수 있습니다.
            return Err(io::Error::new(io::ErrorKind::Other, "Stream already set"));
        }

        // TcpStream 생성 및 설정
        let stream = TcpStream::connect(address).await?;
        self.connectStream = Some(stream);
        Ok(())
    }    

    async fn send_data(&mut self, data: &[u8]) -> io::Result<usize> {
        if let Some(ref mut stream) = self.connectStream {
            // Some(TcpStream)인 경우에만 데이터를 보낼 수 있도록 처리
            stream.write_all(data).await?;
            Ok(data.len())
        } else {
            // None인 경우 에러 처리
            Err(io::Error::new(io::ErrorKind::Other, "Stream not set"))
        }
    }    

    pub fn SendMessageToServer(&mut self, msg: &str) {
        println!("Input: {}", msg);
        // msg를 서버로 전송함
    }

    pub fn Exit(&mut self) {
        println!("Exit Message Prompt.");
        // 서버와의 연결을 종료함
    }

    pub fn Wait(&mut self){
        println!("Wait . . . ");
    }

    pub fn Run(&mut self) {
        let mut input_buffer = String::new();

        loop {
            // 입력 대기
            print!("Please Input Command : ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input_buffer).expect("Input Error");

            let input = input_buffer.trim();
            // 이스케이프 문자를 받았다면 프로그램을 종료

            // let command_from_str: &str = "exit";
            let converted_command: PromptCommand = input.into();
            self.ExecuteCommand(converted_command);
            // if input == "exit" {
            //     self.Exit();
            //     break;
            // }
    
            // 입력을 출력합니다.
            // self.SendMessageToServer(input);
    
            // 버퍼를 다시 비웁니다.
            input_buffer.clear();
        }
    }

}
