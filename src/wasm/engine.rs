use crate::config::loader::PluginCaps;
use crate::config::loader::{PluginDirPerms, PluginFilePerms};
use crate::{SkyforgeApi, WasmPlugin};
use anyhow::Result;
use tracing::info;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

impl From<PluginDirPerms> for DirPerms {
    fn from(perm: PluginDirPerms) -> Self {
        let mut file_perms = DirPerms::empty();
        if perm.read {
            file_perms.insert(DirPerms::READ);
        }
        if perm.write {
            file_perms.insert(DirPerms::MUTATE);
        }
        file_perms
    }
}

impl From<PluginFilePerms> for FilePerms {
    fn from(perm: PluginFilePerms) -> Self {
        let mut file_perms = FilePerms::empty();
        if perm.read {
            file_perms.insert(FilePerms::READ);
        }
        if perm.write {
            file_perms.insert(FilePerms::WRITE);
        }
        file_perms
    }
}

pub struct MyState {
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

    let mut store = create_store(
        &engine,
        vec![
            PluginCaps::InheritArgs,
            PluginCaps::InheritEnv,
            PluginCaps::InheritStdio,
            PluginCaps::InheritStdout,
        ],
    );

    let component = Component::from_file(&engine, &plugin.path)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let skyforge = SkyforgeApi::instantiate(&mut store, &component, &linker)?;

    plugin.instance = Some(skyforge);
    info!("Plugin {} loaded successfully!", plugin.name);

    Ok(())
}

pub fn create_store(engine: &Engine, capabilities: Vec<PluginCaps>) -> Store<MyState> {
    let mut builder = WasiCtxBuilder::new();

    for cap in capabilities.into_iter() {
        match cap {
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
