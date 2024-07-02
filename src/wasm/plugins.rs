use crate::init_plugin;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::error;
use wasmtime::Instance;

#[derive(Clone, Debug)]
pub struct WasmPlugin {
    pub name: String,
    pub path: PathBuf,
    pub instance: Option<Instance>,
}

pub async fn load_plugin<P: AsRef<Path>>(config_path: P, name: &str) -> Result<WasmPlugin> {
    if let Some(mut plugin) = read_plugins(config_path.as_ref().join("plugins"), name)
        .await
        .with_context(|| {
            format!(
                "Failed to read plugins from {:?}",
                config_path.as_ref().join("plugins")
            )
        })?
    {
        init_plugin(&mut plugin)
            .with_context(|| format!("Failed to init plugin: {}", plugin.name))?;
        Ok(plugin)
    } else {
        anyhow::bail!(
            "No plugins found in {:?}",
            config_path.as_ref().join("plugins")
        );
    }
}
// TODO: make this function return just Option<WasmpPlugin> instead of Result<Option<WasmPlugin>>
async fn read_plugins(modules_path: PathBuf, plugin_name: &str) -> Result<Option<WasmPlugin>> {
    let mut dir = fs::read_dir(&modules_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let file_name_opt = path.file_name().and_then(|os_str| os_str.to_str());

        if let Some(file_name) = file_name_opt {
            if file_name.ends_with(".wasm") {
                if file_name.contains(plugin_name) {
                    return Ok(Some(WasmPlugin {
                        name: file_name.to_string(),
                        path,
                        instance: None,
                    }));
                }
            }
        } else {
            error!("Invalid file name for path: {:?}", path);
        }
    }
    Ok(None)
}
