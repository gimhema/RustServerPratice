pub mod GameStructureModule;
pub mod GameUnitModule;
pub mod GmaeResourceSystemModule;
pub mod GameNonPlayerableModules;

use crate::ArenaServerModule;
use crate::CommonModule;
use crate::CommonModule::Manager::{Manager};


pub mod GameLogicCore {
    use crate::ArenaServerModule::ArenaServerActionMappingModule::ServerActionMappingInitialize;
    use crate::GameLogicCore::GameStructureModule;
    use crate::GameLogicCore::GameUnitModule;
    use crate::GameLogicCore::GmaeResourceSystemModule;
    use crate::GameLogicCore::GameNonPlayerableModules;

    use super::GameNonPlayerableModules::GameNPCManager;
    use super::GameNonPlayerableModules::GameNonPlayerableEcoSystem;
    use super::GameStructureModule::GameStructureManager;
    use super::GameUnitModule::GameUnitManager;
    use super::GmaeResourceSystemModule::GameResourceSysManager;
    use super::CommonModule::Manager::{Manager};

    use super::ArenaServerModule::ArenaServerCoreModule;

    pub fn ModInitialize() {
        

     }

     pub fn GameLogicCoreFunctionTest() {
        println!("TEST");
     }

   pub struct GameMainLogic {
      id : i64,
      gameStructureManager : GameStructureManager,
      gmaeUnitManager : GameUnitManager,
      gameResourcrSysManager : GameResourceSysManager
   }

   impl Manager for GameMainLogic {
      fn Initialize(&self) {
         println!("Initailize Game Main Logic . . . ");
         self.gameStructureManager.Initialize();
         self.gmaeUnitManager.Initialize();
         self.gameResourcrSysManager.Initialize();
     }
     fn Update(&self) {
         println!("Game Main Logic Update");
         self.gameStructureManager.Update();
         self.gmaeUnitManager.Update();
         self.gameResourcrSysManager.Update();

     }
   }

   pub struct GamePlayer {
      id : i64,
      gameMainLogic : GameMainLogic
   }

   impl Manager for GamePlayer {
      fn Initialize(&self) {
          self.gameMainLogic.Initialize();
      }
      fn Update(&self) {
          self.gameMainLogic.Update();
      }
   }

   pub struct GameEnvironment {
      id : i64,
      gameNPCManager : GameNPCManager,
      gameEcosystem : GameNonPlayerableEcoSystem
   }

   impl Manager for GameEnvironment {
      fn Initialize(&self) {
          
      }
      fn Update(&self) {
          
      }
   }



}