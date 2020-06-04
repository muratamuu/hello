extern crate hello;

use hello::{config, controller};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = config::Config::parse_cmd_line()?;

    controller::run(config).await?;

    Ok(())
}
