use tokio::{
    io::{ AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener,
    sync::broadcast,
};

fn PrintStr( data: &mut String ){
    let printStr = data;
    println!("Server Received Data : {}", printStr);
}

#[tokio::main]
async fn main() {

    println!("Server Running . . . .");

    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
    
            let mut reader = BufReader::new(reader);    
            let mut line = String::new();
    
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                    
                        if result.unwrap() == 0 {
                            break;
                        }
                        PrintStr(&mut line);
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();                    
                    }    
                }
            }
        });

    }

}
