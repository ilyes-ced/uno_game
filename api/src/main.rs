use tokio::io::AsyncBufReadExt;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;




#[tokio::main]
async fn main() {
    println!("hello");

    let listener: TcpListener = TcpListener::bind("localhost:5000").await.unwrap();


   

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("user address: {}",addr);
    
        tokio::spawn(async move{

            let (reader, mut writer) = socket.split();
    
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
    
            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap(); 
                if bytes_read == 0 {
                    break;
                }
                writer.write_all(&line.as_bytes()).await.unwrap();
                line.clear();
            }
        });

    }



}
