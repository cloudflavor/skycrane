use crate::starlark::std::PluginCapabilities;
use crate::SkyforgeApi;
use crate::{init_plugin, starlark::std::CloudModule};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error};

pub struct WasmPlugin {
    pub name: String,
    pub path: PathBuf,
    pub instance: Option<SkyforgeApi>,
    pub capabilities: Vec<PluginCapabilities>,
}

pub async fn load_plugin(
    config_path: impl AsRef<Path>,
    cloud_module: &CloudModule,
) -> Result<WasmPlugin> {
    let plugins_path = config_path.as_ref().join("plugins");
    debug!("Loading plugins from {:?}", plugins_path);

    if let Ok(mut plugin) = read_plugins(plugins_path.clone(), cloud_module.name.as_str()).await {
        init_plugin(&mut plugin, cloud_module)
            .with_context(|| format!("Failed to init plugin: {}", plugin.name))?;

        Ok(plugin)
    } else {
        anyhow::bail!("No plugins found in {:?}", &plugins_path,);
    }
}

async fn read_plugins(modules_path: impl AsRef<Path>, plugin_name: &str) -> Result<WasmPlugin> {
    let mut dir = fs::read_dir(&modules_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let file_name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or(anyhow::anyhow!("Invalid file name for path: {:?}", path))?;

        if file_name.ends_with(".wasm") && file_name.contains(plugin_name) {
            return Ok(WasmPlugin {
                name: file_name.to_string(),
                path,
                instance: None,
                capabilities: Vec::new(),
            });
        } else {
            error!("Invalid file name for path: {:?}", path);
        }
    }
    Err(anyhow::anyhow!("No plugin {plugin_name} found!"))
}
