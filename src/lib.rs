#[macro_use]
extern crate clap;

use std::error::Error;
use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};
use tokio::stream::StreamExt;

pub mod config;

pub async fn run(config: config::Config) -> Result<(), Box<dyn Error>> {

    let addr = format!("0.0.0.0:{}", config.listen_port).to_string();

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
}
