use crate::config::loader::{extract_plugin, load, read_config};
use crate::Init;
use anyhow::Result;
use std::path::Path;
use tracing::info;

// Loading phase
// * parse all the files from the directory that was passed by the user.
// * establish what plugin we need based on the module name and version
// * instantiate the plugin from the plugin path e.g.: ~/.config/skycrane/plugins/
// * send starlark config to wasi plugin
// * wasi plugin parses the config, filters out only available functionality it
//   has, ignores everything else. It's more than possible that the contents of
//   the starlark file are mixed between functions that the rust CLI offers
//   and functions that the WASI plugin offers. Evaluating a starlark file
//   with functions that are not in globals, will panic.

pub async fn init(opts: Init, config_path: impl AsRef<Path>) -> Result<()> {
    info!("Initializing new repository at {:?}", opts.path);

    let base = format!("{}/base.star", opts.path.display());
    let config = read_config(&base).await?;
    let cloud_mod = extract_plugin(config.as_str())?;
    let _ = load(config_path, &cloud_mod.name).await?;

    Ok(())
}
