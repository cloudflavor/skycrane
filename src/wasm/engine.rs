use crate::WasmPlugin;
use anyhow::{Context, Ok, Result};
use tracing::info;
use wasi_common::sync::{add_to_linker, WasiCtxBuilder};
use wasmtime::*;

pub fn init_plugin(plugin: &mut WasmPlugin) -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker, |s| s).with_context(|| "Failed to add WASI to linker")?;
    info!("Added WASI to linker");
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().inherit_env()?.build();
    let mut store = Store::new(&engine, wasi_ctx);

    let module = Module::from_file(&engine, &plugin.path)?;
    linker.module(&mut store, &plugin.name, &module)?;
    let instance = linker.instantiate(&mut store, &module)?;
    instance
        .get_func(&mut store, "plugin-api#deserialize-config")
        .ok_or(anyhow::anyhow!(
            "Failed to find deserialize-config function"
        ))?;

    info!(
        "Successfully validated: {} from: {}",
        plugin.name,
        plugin.path.display()
    );

    plugin.instance = Some(instance);

    Ok(())
}
