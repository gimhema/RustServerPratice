



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

    pub fn Wait(&mut self){
        println!("Wait . . . ");
    }

    pub fn Run(&mut self) {

    }

}
