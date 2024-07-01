use anyhow::{Context, Result};
use rust_embed::RustEmbed;
use skycrane::{
    apply, destroy, init, load_plugins, reconcile, Cli, Commands, PluginsSchema, INTERFACE_SCHEMA,
};
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
    load_schema().await?;

    let instances = load_plugins(opts.config_path.clone())
        .await
        .with_context(|| {
            format!(
                "Failed to load plugin from {:?}",
                opts.config_path.join("plugins")
            )
        })?;

    match opts.commands {
        Commands::Init(_) => init(&opts, instances.first().unwrap().to_owned()).await?, // TODO: Fix me!
        Commands::Apply(_) => apply(&opts)?,
        Commands::Reconcile(_) => reconcile(&opts)?,
        Commands::Destroy(_) => destroy(&opts)?,
    }

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "$SKYCRANE_SPEC_PATH"]
struct Assets;

async fn load_schema() -> Result<()> {
    let spec_file = Assets::get("plugins-interface.json")
        .with_context(|| "Failed to load plugins-interface.json from virtual fs")?;

    let data = spec_file.data.as_ref();

    let plugins_schema = serde_json::from_slice::<PluginsSchema>(data)
        .with_context(|| "Failed to parse interface schema")?;

    INTERFACE_SCHEMA.set(plugins_schema).unwrap(); // TODO: Fix me!
    debug!("Loaded interface schema: {:#?}", INTERFACE_SCHEMA);

    Ok(())
}
