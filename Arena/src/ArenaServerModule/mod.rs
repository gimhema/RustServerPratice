pub mod ArenaClientModule;
pub mod ArenaCommonModule;

pub mod ArenaServerCoreModule {
    use crate::ArenaServerModule::ArenaCommonModule;
    use crate::ArenaServerModule::ArenaClientModule;

    use super::ArenaCommonModule::MessageUnique;

    pub fn Create() {
        println!("Server Core Create");
        ArenaCommonModule::Initialize();
        ArenaClientModule::Initailize();
    }

    pub fn FunctionTest(){
        let mut _unique:MessageUnique = MessageUnique::NONE;
        let mut _str:String = ArenaCommonModule::ConvertUniqueToData(_unique);

        println!("Result : {}", _str);
    }
}

pub mod ArenaGameCommon {

    use crate::sendMessageBuffer;

    use super::ArenaClientModule::ArenaPlayer; 

    pub struct InstanceGame {
        gameID: i32,
        players: Vec<ArenaPlayer>
    }

    impl InstanceGame {
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




