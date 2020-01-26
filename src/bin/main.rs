extern crate hello;

use hello::{config, server};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = config::Config::parse_cmd_line()?;

    server::run(config).await?;

    Ok(())
}
