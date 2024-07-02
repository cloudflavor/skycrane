use crate::init_plugins;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error};
use wasmtime::Instance;

#[derive(Clone, Debug)]
pub struct WasmPlugin {
    pub name: String,
    pub path: PathBuf,
}

pub async fn load_plugins<P: AsRef<Path>>(config_path: P) -> Result<Vec<Instance>> {
    let plugins = read_plugins(config_path.as_ref().join("plugins")).await?;

    if plugins.is_empty() {
        anyhow::bail!("No plugins found in {:?}", config_path.as_ref());
    }

    debug!("Found {} wasm plugin(s): {:#?}", plugins.len(), plugins);

    init_plugins(plugins)
}

async fn read_plugins(modules_path: PathBuf) -> Result<Vec<WasmPlugin>> {
    let mut dir = fs::read_dir(&modules_path).await?;
    let mut modules = Vec::new();

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let file_name_opt = path.file_name().and_then(|os_str| os_str.to_str());

        if let Some(file_name) = file_name_opt {
            if file_name.ends_with(".wasm") {
                modules.push(WasmPlugin {
                    name: file_name.to_string(),
                    path,
                });
            }
        } else {
            error!("Invalid file name for path: {:?}", path);
        }
    }

    Ok(modules)
}
