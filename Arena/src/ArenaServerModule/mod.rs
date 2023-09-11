pub mod ArenaClientModule;
pub mod ArenaMessageModule;
pub mod ArenaGameCommonModule;
pub mod ArenaServerActionModule;
pub mod ArenaServerActionMappingModule;


use crate::GameLogicCore;

pub mod ArenaServerCoreModule {
    use crate::ArenaServerModule::ArenaMessageModule;
    use crate::ArenaServerModule::ArenaClientModule;
    use crate::ArenaServerModule::ArenaServerActionMappingModule::ServerActionMappingInitialize;
    use super::ArenaMessageModule::MessageUnique;
    use super::GameLogicCore::GameLogicCore;
    use super::ArenaServerActionMappingModule;
    use super::ArenaGameCommonModule::InstanceGame;
    use super::ArenaClientModule::ArenaPlayer;

    pub fn ArenaServerInitialize() {

    }

//    pub fn 

    pub fn ServerCoreFunctionTest(){
        let mut _unique:MessageUnique = MessageUnique::NONE;
        let mut _str:String = ArenaMessageModule::ConvertUniqueToData(_unique);

        println!("Result : {}", _str);

        GameLogicCore::GameLogicCoreFunctionTest();
    }
}



