use crate::init_plugin;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error};
use wasmtime::Instance;

#[derive(Clone, Debug)]
pub struct WasmPlugin {
    pub name: String,
    pub path: PathBuf,
    pub instance: Option<Instance>,
}

pub async fn load_plugin<P: AsRef<Path>>(config_path: P, name: &str) -> Result<WasmPlugin> {
    let plugins_path = config_path.as_ref().join("plugins");
    debug!("Loading plugins from {:?}", plugins_path);

    if let Ok(mut plugin) = read_plugins(plugins_path.clone(), name).await {
        init_plugin(&mut plugin)
            .with_context(|| format!("Failed to init plugin: {}", plugin.name))?;

        Ok(plugin)
    } else {
        anyhow::bail!("No plugins found in {:?}", &plugins_path,);
    }
}
async fn read_plugins(modules_path: PathBuf, plugin_name: &str) -> Result<WasmPlugin> {
    let mut dir = fs::read_dir(&modules_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let file_name_opt = path.file_name().and_then(|os_str| os_str.to_str());

        if let Some(file_name) = file_name_opt {
            if file_name.ends_with(".wasm") {
                if file_name.contains(plugin_name) {
                    return Ok(WasmPlugin {
                        name: file_name.to_string(),
                        path,
                        instance: None,
                    });
                }
            }
        } else {
            error!("Invalid file name for path: {:?}", path);
        }
    }

    return Err(anyhow::anyhow!("No plugin {plugin_name} found!"));
}
