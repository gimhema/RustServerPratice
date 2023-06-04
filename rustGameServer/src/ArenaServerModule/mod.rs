mod ArenaClientModule;
mod ArenaCommonModule;

pub mod ArenaServerCoreModule {
    use crate::ArenaServerModule::ArenaCommonModule;
    use crate::ArenaServerModule::ArenaNetworkModule;
    use crate::ArenaServerModule::ArenaWorldModule;
    use crate::ArenaServerModule::ArenaMessageModule;
    use crate::ArenaServerModule::ArenaClientModule;

    use super::ArenaCommonModule::MessageUnique;

    pub fn Create() {
        println!("Server Core Create");
        ArenaCommonModule::Initialize();
        ArenaNetworkModule::Initialize();
        ArenaWorldModule::Initialize();
        ArenaMessageModule::Initialize();
        ArenaClientModule::Initailize();
    }

    pub fn FunctionTest(){
        let mut _unique:MessageUnique = MessageUnique::NONE;
        let mut _str:String = ArenaCommonModule::ConvertUniqueToData(_unique);

        println!("Result : {}", _str);
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




