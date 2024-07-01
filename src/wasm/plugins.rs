use crate::init_plugins;
use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error};
use wasmtime::{Instance, Store};

pub static INTERFACE_SCHEMA: OnceCell<PluginsSchema> = OnceCell::new();

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginsSchema {
    funcs: Vec<FunctionSpec>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionSpec {
    name: String,
    params: HashMap<String, String>,
    returns: String,
}

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

pub fn validate_interface(store: &mut Store<()>, instance: &Instance) -> Result<()> {
    let schema = INTERFACE_SCHEMA.get().context("interface schema not set")?;

    for func_spec in &schema.funcs {
        let func = instance
            .get_func(&mut *store, &func_spec.name)
            .with_context(|| {
                format!(
                    "failed to get function {} from plugin {:?}",
                    func_spec.name, instance,
                )
            })?;

        let ty = func.ty(&store);
        let params = ty.params();
        params.into_iter().enumerate().try_for_each(|(i, ty)| {
            let param_key = i.to_string();
            let expected_ty_str = func_spec.params.get(&param_key).context(format!(
                "parameter key {param_key} not found in function specification"
            ))?;

            if *expected_ty_str != ty.to_string() {
                return Err(anyhow::anyhow!(
                    "parameter type mismatch, expected {expected_ty_str}, got {}",
                    ty.to_string()
                ));
            }

            Ok(())
        })?;

        let return_type = ty.results().next();
        if let Some(return_type) = return_type {
            let returns = func_spec.returns.as_str();
            if return_type.to_string() != returns {
                return Err(anyhow::anyhow!(
                    "return type mismatch, expected {returns}, got {}",
                    return_type.to_string(),
                ));
            }
        }
    }

    Ok(())
}
