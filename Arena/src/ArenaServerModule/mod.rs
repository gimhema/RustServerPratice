pub mod ArenaClientModule;
pub mod ArenaMessageModule;
pub mod ArenaGameCommonModule;

use crate::GameLogicCore;

pub mod ArenaServerCoreModule {
    use crate::ArenaServerModule::ArenaMessageModule;
    use crate::ArenaServerModule::ArenaClientModule;
    use super::ArenaMessageModule::MessageUnique;

    use super::GameLogicCore::GameLogicCore;

    pub fn Create() {
        println!("Server Core Create");
        ArenaMessageModule::Initialize();
        ArenaClientModule::Initailize();
    }

    pub fn ServerCoreFunctionTest(){
        let mut _unique:MessageUnique = MessageUnique::NONE;
        let mut _str:String = ArenaMessageModule::ConvertUniqueToData(_unique);

        println!("Result : {}", _str);

        GameLogicCore::GameLogicCoreFunctionTest();
    }
}



