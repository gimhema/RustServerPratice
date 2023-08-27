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

   impl GameMainLogic {
      pub fn new(gid: i64) -> Self {
         // GameUnitManager 초기화
         let game_unit_manager = GameUnitManager::new(gid); // GameUnitManager 생성
         let game_struture_manager = GameStructureManager::new(gid);
         let game_resource_sys_manager = GameResourceSysManager::new(gid);
         // GameMainLogic 구조체 생성 및 반환
         GameMainLogic {
             id: gid,
             gameStructureManager: game_struture_manager,
             gmaeUnitManager: game_unit_manager,
             gameResourcrSysManager: game_resource_sys_manager
         }
     }
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

   pub struct GamePlayerLogic {
      id : i64,
      gameMainLogic : GameMainLogic
   }

   impl GamePlayerLogic {
      pub fn new(gid: i64) -> GamePlayerLogic {
         let game_main_logic = GameMainLogic::new(gid);
         GamePlayerLogic {
            id: gid,
            gameMainLogic: game_main_logic
         }
      }
   }

   impl Manager for GamePlayerLogic {
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