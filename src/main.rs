extern crate hello;

use hello::{run, config};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = config::Config::parse_cmd_line()?;

    run(config).await?;

    Ok(())
}
