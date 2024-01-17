use anyhow::Result;
use skycrane::Cli;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Cli::from_args();

    Ok(())
}
