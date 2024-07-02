use anyhow::{Context, Result};
use skycrane::{apply, destroy, init, load_plugin, reconcile, Cli, Commands};
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

    let plugin = load_plugin(opts.config_path.clone(), "hetzner") // TODO: Fix me, this should come from the starlark file.
        .await
        .with_context(|| {
            format!(
                "Failed to load plugin from {:?}",
                opts.config_path.join("plugins")
            )
        })?;

    match opts.commands {
        Commands::Init(_) => init(&opts, plugin).await?, // TODO: Fix me!
        Commands::Apply(_) => apply(&opts)?,
        Commands::Reconcile(_) => reconcile(&opts)?,
        Commands::Destroy(_) => destroy(&opts)?,
    }

    Ok(())
}
