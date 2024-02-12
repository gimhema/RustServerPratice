use super::PromptClientMainModule::*;



pub enum PromptCommand {
    COMMAND_REQUEST_CONNECT,
    COMMAND_REQUEST_EXIT,    
} 

impl From<PromptCommand> for String {
    fn from(header: PromptCommand) -> Self {
        match header {
            PromptCommand::COMMAND_REQUEST_CONNECT => "connect".to_string(),
            PromptCommand::COMMAND_REQUEST_EXIT => "exit".to_string(),
        }
    }
}


impl PromptClientMain {
    pub fn ExecuteCommand(&mut self, _command : PromptCommand) {

    }
}

