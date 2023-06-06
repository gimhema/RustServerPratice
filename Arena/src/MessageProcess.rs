
pub mod MessageProcess {

// 클라이언트로부터 받은 메세지를 처리한다.
// 서버는 받은 메세지에 따라서 동작을 수행함 . . . .
    enum MessageType {
        Echo,
        SendAll,
    }

    pub fn DataToMessage( val: &str ) -> MessageType {
        // 클라이언트로부터 받은 데이터를 메세지로 변환

    }

    pub fn MessageToData( val: MessageType ) -> String {
        // 메세지를 클라이언트로 보낼 수 있는 데이터의 형식으로 변환함
    }
    
}

pub mod RequestProcess {

}

pub mod ResponseProcess {
    
}