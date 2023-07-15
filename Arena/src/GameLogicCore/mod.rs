pub mod GameStructureModule;
pub mod GameUnitModule;
pub mod GmaeResourceSystemModule;

use crate::ArenaServerModule;


pub mod GameLogicCore {
    use crate::GameLogicCore::GameStructureModule;
    use crate::GameLogicCore::GameUnitModule;
    use crate::GameLogicCore::GmaeResourceSystemModule;

    use super::GameStructureModule::GameStructureManager;
    use super::GameUnitModule::GameUnitManager;
    use super::GmaeResourceSystemModule::GameResourceSysManager;

    use super::ArenaServerModule::ArenaServerCoreModule;

    pub fn Initialize() {
        ArenaServerCoreModule::ServerCoreFunctionTest();
     }

     pub fn GameLogicCoreFunctionTest() {
        println!("TEST");
     }

}