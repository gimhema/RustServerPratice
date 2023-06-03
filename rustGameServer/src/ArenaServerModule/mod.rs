
pub mod ArenaServerCoreModule {
    use crate::ArenaServerModule::ArenaCommonModule;
    use crate::ArenaServerModule::ArenaNetworkModule;
    use crate::ArenaServerModule::ArenaWorldModule;
    use crate::ArenaServerModule::ArenaMessageModule;

    pub fn Create() {
        println!("Server Core Create");
        ArenaCommonModule::Initialize();
        ArenaNetworkModule::Initialize();
        ArenaWorldModule::Initialize();
        ArenaMessageModule::Initialize();

    }
}

pub mod ArenaClientModule {
    pub fn Initialize() {
        
    }
}

pub mod ArenaCommonModule {
    pub enum MessageUnique {
        NONE,
        REQUEST_SEND_ONE,
        REQUEST_SEND_ALL,
        RESPONSE_SEND_ONE,
        RESPONSE_SEND_ALL,
        ERROR,
    }

    pub fn Initialize() {
        println!("Common Module Initialize . . . .");
    }

    pub fn ConvertDataToUnique(_data: &str) -> MessageUnique {
        match _data {
            "NONE" => MessageUnique::NONE,
            "REQUEST_SEND_ONE" => MessageUnique::REQUEST_SEND_ONE,
            "REQUEST_SEND_ALL" => MessageUnique::REQUEST_SEND_ALL,
            "RESPONSE_SEND_ONE" => MessageUnique::RESPONSE_SEND_ONE,
            "RESPONSE_SEND_ONE" => MessageUnique::RESPONSE_SEND_ONE,
            _ => MessageUnique::ERROR
        }
    }
    
}

pub mod ArenaNetworkModule {
    pub fn Initialize() {
        println!("Network Module Initialize . . . .");

    }
}

pub mod ArenaWorldModule {
    pub fn Initialize() {
        println!("Message Module Initialize . . . .");

    }    
}

pub mod ArenaMessageModule {
    pub fn Initialize() {
        println!("Message Module Initialize . . . .");

    }
}




