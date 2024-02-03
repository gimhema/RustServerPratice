

use std::io::{self, Write};

pub struct PromptClientMain {
    id: i64,
    isConnect: bool,
}

impl PromptClientMain {
    
    pub fn new(_id: i64) -> Self {

        PromptClientMain{
            id: _id,
            isConnect: false
        }
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
            if input == "exit" {
                self.Exit();
                break;
            }
    
            // 입력을 출력합니다.
            self.SendMessageToServer(input);
    
            // 버퍼를 다시 비웁니다.
            input_buffer.clear();
        }
    }

}
