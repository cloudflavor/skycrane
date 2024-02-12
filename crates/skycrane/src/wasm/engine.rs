use crate::wasm::plugins::validate_interface;
use crate::WasmPlugin;
use anyhow::Result;
use tracing::error;
use wasmtime::{Engine, Instance, Module, Store};

pub fn init_plugins(plugins: Vec<WasmPlugin>) -> Result<Vec<Instance>> {
    let mut instances = Vec::new();
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    for plugin in plugins {
        let module = Module::from_file(&engine, &plugin.path)?;
        // NOTE: maybe at some point we want to pass in some imports for the plugins
        let instance = Instance::new(&mut store, &module, &[])?;
        if let Err(err) = validate_interface(&mut store, &instance) {
            error!(
                "Failed to validate interface for plugin {:?}: {err}",
                plugin.name
            );
            continue;
        }
        instances.push(instance);
    }

    Ok(instances)
}
