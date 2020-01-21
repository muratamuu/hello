use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};
use tokio::stream::StreamExt;
use std::error::Error;

// コメントテスト
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080".to_string();

    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut framed = BytesCodec::new().framed(socket);

            while let Some(message) = framed.next().await {
                match message {
                    Ok(bytes) => println!("bytes: {:?}", bytes),
                    Err(err) => println!("Socket closed with error: {:?}", err),
                }
            }
            println!("Socket received FIN packet and closed connection");
        });
    }

    // let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    // println!("created stream");

    // let result = stream.write(b"hello world\n").await;
    // println!("wrote to stream; success={:?}", result.is_ok());

    // Ok(())
}
