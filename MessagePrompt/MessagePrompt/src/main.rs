mod PromptClient;
use PromptClient::PromptClientMainModule::PromptClientMain;

fn main() {
    println!("Hello, world!");

    let mut pClient = PromptClientMain::new(0);

    pClient.Start();

}
