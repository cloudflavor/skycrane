use crate::WasmPlugin;
use anyhow::{Context, Result};
use tracing::info;
use wasi_common::sync::{add_to_linker, WasiCtxBuilder};
use wasmtime::*;

pub fn init_plugins(plugins: Vec<WasmPlugin>) -> Result<Vec<Instance>> {
    let mut instances = Vec::new();
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker, |s| s).with_context(|| "Failed to add WASI to linker")?;
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().inherit_env()?.build();
    let mut store = Store::new(&engine, wasi_ctx);

    for plugin in plugins {
        let module = Module::from_file(&engine, &plugin.path)?;
        linker.module(&mut store, &plugin.name, &module)?;
        let instance = linker.instantiate(&mut store, &module)?;
        instance
            .get_func(&mut store, "plugin-api#deserialize-config")
            .unwrap();

        info!(
            "Successfully validated: {} from: {}",
            plugin.name,
            plugin.path.display()
        );

        instances.push(instance);
    }

    Ok(instances)
}
