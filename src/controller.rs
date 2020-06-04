use super::server;
use std::error::Error;

pub async fn run(config: super::config::Config) -> Result<(), Box<dyn Error>> {

    server::run(config).await?;

    Ok(())
}
