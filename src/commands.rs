use crate::config::loader::{extract_plugin, load, read_config};
use crate::Init;
use anyhow::Result;
use std::path::Path;
use tracing::info;

pub async fn init(opts: Init, config_path: impl AsRef<Path>) -> Result<()> {
    info!("Initializing new repository at {:?}", opts.path);

    let base = format!("{}/base.star", opts.path.display());
    let config = read_config(&base).await?;
    let plugin = extract_plugin(config.as_str())?;
    load(config_path, &plugin.name).await?;

    Ok(())
}
