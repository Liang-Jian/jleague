// src/main.rs
mod config;
mod db;
mod fetch;
mod parse;

use config::Config;
use db::init_db;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cfg = Config::from_file("run.yml")?;
    init_db(&cfg.db)?;
    fetch::process_all_matches().await?;
    Ok(())
}
