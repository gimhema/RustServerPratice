pub mod ArenaClientModule;
pub mod ArenaCommonModule;

use crate::GameLogicCore;

pub mod ArenaServerCoreModule {
    use crate::ArenaServerModule::ArenaCommonModule;
    use crate::ArenaServerModule::ArenaClientModule;
    use super::ArenaCommonModule::MessageUnique;

    use super::GameLogicCore::GameLogicCore;

    pub fn Create() {
        println!("Server Core Create");
        ArenaCommonModule::Initialize();
        ArenaClientModule::Initailize();
    }

    pub fn ServerCoreFunctionTest(){
        let mut _unique:MessageUnique = MessageUnique::NONE;
        let mut _str:String = ArenaCommonModule::ConvertUniqueToData(_unique);

        println!("Result : {}", _str);

        GameLogicCore::GameLogicCoreFunctionTest();
    }
}

pub mod ArenaGameCommon {

    use crate::{sendMessageBuffer, recvMessageBuffer};

    use super::ArenaClientModule::ArenaPlayer; 
    use super::ArenaCommonModule::{self, MakeSendMessage};

    pub struct InstanceGame {
        gameID: i32,
        players: Vec<ArenaPlayer>
    }

    // Async
    // InstanceGame의 GameWait 이후 시작
    pub fn RecvMessageProcessLoop() {
        loop {
            // recvMessageBuffer.lock().unwrap() 에서 메세지를 꺼낸다.

            // 메세지를 꺼내고 헤더에 맞는 콜백 함수를 실행시킨다.

            // 콜백함수에는 헤더 이후에 작성된 정보들이 저장되어있다.
        
        }
    }

    pub fn PushSendMessageToSendBuffer(_header : i64, _command : i64, _param : String) {

//        MakeSendMessage(header, command, param)를 만들고
        // sendBuffer에 메세지를 저장한다.

    }

    impl InstanceGame {
        
        // 게임 흐름
        pub fn GameCreate() {
            // 게임 생성
        }
    
        pub fn GameWait() {
            // 게임 생성 후 플레이어의 준비 대기
        }

        pub fn GameStart() {
            // 게임 시작
        }
    
        pub fn GameAction() {
            // 루프를 돌면서 게임의 로직을 실행
            loop {
                sendMessageBuffer.lock().unwrap().push_back("test".to_string());
            }
        }
    
        pub fn GameEnd() {
            // 게임을 끝내고 결과를 플레이어들에게 전송한다.
        }
    
        pub fn GameClose() {
            // 게임을 닫고 대기상태로 돌아간다.
        }
    
        pub fn GameDestroy() {
            // 인스턴스 게임을 파괴한다.
        }


    }
}




