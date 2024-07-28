use anyhow::Result;
use skycrane::{init, Cli, Commands};
use structopt::StructOpt;
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Cli::from_args();

    let opts_level = opts.log_level.clone();
    let log_level = EnvFilter::new(opts_level);

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(log_level)
        .with_ansi(true)
        .with_writer(std::io::stdout)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    debug!("loading with opts: {:#?}", &opts);

    match opts.commands {
        Commands::Init(args) => init(args, &opts.config_path).await?,
        Commands::Plan(_) => unimplemented!("Plan not implemented"),
        Commands::Apply(_) => unimplemented!("Apply not implemented"),
        Commands::Reconcile(_) => unimplemented!("Reconcile not implemented"),
        Commands::Destroy(_) => unimplemented!("Destroy not implemented"),
    }

    Ok(())
}
