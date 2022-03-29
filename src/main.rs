mod cli;
mod fetch;
mod formatter;

#[macro_use]
extern crate log;
extern crate clap;

use cli::{Cli, Result};
use env_logger::Env;
use fetch::Problems;
use once_cell::sync::OnceCell;
use tokio;

static PROBLEMS: OnceCell<Problems> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warning")).init();

    let mut cli = Cli::init().await.unwrap();
    cli.run().await?;

    Ok(())
}
