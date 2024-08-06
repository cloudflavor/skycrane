use crate::{SkyforgeApi, WasmPlugin};
use anyhow::Result;
use tracing::info;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

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

    let mut store = create_store(vec![
        PluginCaps::InheritEnv,
        PluginCaps::InheritStdio,
        PluginCaps::InheritStdout,
    ]);

    let component = Component::from_file(&engine, &plugin.path)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let instance = linker.instantiate(&mut store, &component)?;
    let skyforge = SkyforgeApi::new(&mut store, &instance)?;

    plugin.instance = Some(skyforge);
    info!("Plugin {} loaded successfully!", plugin.name);

    Ok(())
}

pub enum PluginCaps {
    CanMountPaths(WasmPluginPath),
    InheritArgs,
    InheritEnv,
    InheritStdio,
    InheritStdout,
}

pub struct WasmPluginPath {
    pub host_path: String,
    pub guest_path: String,
    pub dir_perms: DirPerms,
    pub file_perms: FilePerms,
}

pub fn create_store(capabilities: Vec<PluginCaps>) -> Store<MyState> {
    let engine = Engine::default();
    let mut builder = WasiCtxBuilder::new();

    for cap in capabilities.into_iter() {
        match cap {
            PluginCaps::CanMountPaths(opts) => {
                builder.preopened_dir(
                    opts.host_path,
                    opts.guest_path,
                    opts.dir_perms,
                    opts.file_perms,
                );
            }
            PluginCaps::InheritArgs => {
                builder.inherit_args();
            }
            PluginCaps::InheritEnv => {
                builder.inherit_env();
            }
            PluginCaps::InheritStdio => {
                builder.inherit_stdio();
            }
            PluginCaps::InheritStdout => {
                builder.inherit_stdout();
            }
        }
    }

    Store::new(
        &engine,
        MyState {
            ctx: builder.build(),
            table: ResourceTable::default(),
        },
    )
}
