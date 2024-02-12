use super::PromptClientMainModule::*;



pub enum PromptCommand {
    COMMAND_REQUEST_DEFAULT,
    COMMAND_REQUEST_CONNECT,
    COMMAND_REQUEST_EXIT,    
} 

impl From<PromptCommand> for &str {
    fn from(header: PromptCommand) -> Self {
        match header {
            PromptCommand::COMMAND_REQUEST_DEFAULT => "",
            PromptCommand::COMMAND_REQUEST_CONNECT => "connect",
            PromptCommand::COMMAND_REQUEST_EXIT => "exit",
        }
    }
}

impl From<&str> for PromptCommand {
    fn from(command : &str) -> Self {
        match command {
            "connect" => PromptCommand::COMMAND_REQUEST_CONNECT,
            "exit" => PromptCommand::COMMAND_REQUEST_CONNECT,
            &_ => PromptCommand::COMMAND_REQUEST_DEFAULT,
        }
    }
}


impl PromptClientMain {
    pub fn ExecuteCommand(&mut self, _command : PromptCommand) {
        match _command {
            PromptCommand::COMMAND_REQUEST_DEFAULT => {

            },
            PromptCommand::COMMAND_REQUEST_CONNECT => {
                self.PromptRequestConnect();
            },
            PromptCommand::COMMAND_REQUEST_EXIT => {
                self.PromptExit();
            }
        }
    }

    pub fn PromptExit(&mut self) {
        self.Exit();        
    }

    pub fn PromptRequestConnect(&mut self ) {

    }
}

