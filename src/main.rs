#[macro_use]
extern crate clap;

use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};
use tokio::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = Config::parse_cmd_line()?;

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

struct Config {
    listen_port: u16,
}

impl Config {
    fn parse_cmd_line() -> Result<Config, Box<dyn Error>> {
        use clap::{App, Arg};

        let app = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(Arg::with_name("listen_port")
                .help("listening port number")
                .short("p")
                .long("port")
                .takes_value(true)
        );

        let matches = app.get_matches();
        let listen_port = matches.value_of("listen_port")
            .unwrap_or("8080")
            .parse()?;

        Ok(Config { listen_port })
    }
}