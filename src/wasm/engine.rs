use crate::{SkyforgeApi, WasmPlugin};
use anyhow::Result;
use tracing::info;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl WasiView for MyState {
    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub fn init_plugin(plugin: &mut WasmPlugin) -> Result<()> {
    let engine = Engine::default();

    let mut store = Store::new(
        &engine,
        MyState {
            // TODO: This should be configurable
            ctx: WasiCtxBuilder::new()
                .inherit_env()
                .inherit_stdio()
                .inherit_stdout()
                .build(),
            table: ResourceTable::default(),
        },
    );
    let component = Component::from_file(&engine, &plugin.path)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let instance = linker.instantiate(&mut store, &component)?;
    let skyforge = SkyforgeApi::new(&mut store, &instance)?;

    plugin.instance = Some(skyforge);
    info!("Plugin {} loaded successfully!", plugin.name);

    Ok(())
}
