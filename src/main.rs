mod cli;
mod fetch;
mod formatter;

use cli::{Cli, Result};
use fetch::Problems;
use once_cell::sync::OnceCell;
use tokio;
use tracing::Level;
use tracing_subscriber;

static PROBLEMS: OnceCell<Problems> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();

    let mut cli = Cli::init().await.unwrap();
    cli.run().await?;

    Ok(())
}
