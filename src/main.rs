extern crate hello;

use hello::{run, Config};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = Config::parse_cmd_line()?;

    run(config).await?;

    Ok(())
}
