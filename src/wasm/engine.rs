use crate::starlark::std::{
    CloudModule, PluginMount, ALLOW_IP_NAME_LOOKUP, INHERIT_ARGS, INHERIT_ENV, INHERIT_NETWORK,
    INHERIT_STDIO, INHERIT_STDOUT,
};
use crate::{SkyforgeApi, WasmPlugin};
use anyhow::{Context, Result};
use tracing::info;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

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

pub fn init_plugin(plugin: &mut WasmPlugin, cloud_module: CloudModule) -> Result<()> {
    let engine = Engine::default();

    let mut store = create_store(&engine, cloud_module).with_context(|| {
        format!(
            "Failed to create store for plugin: {}",
            plugin.name.as_str()
        )
    })?;

    let component = Component::from_file(&engine, &plugin.path)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let skyforge = SkyforgeApi::instantiate(&mut store, &component, &linker)?;

    plugin.instance = Some(skyforge);
    info!("Plugin {} loaded successfully!", plugin.name);

    Ok(())
}

fn add_builder_capabilities(
    mut builder: WasiCtxBuilder,
    capabilities: Vec<String>,
) -> Result<WasiCtxBuilder> {
    for cap in capabilities.into_iter() {
        match cap.as_str() {
            INHERIT_ARGS => {
                builder.inherit_args();
            }
            INHERIT_ENV => {
                builder.inherit_env();
            }
            INHERIT_STDIO => {
                builder.inherit_stdio();
            }
            INHERIT_STDOUT => {
                builder.inherit_stdout();
            }
            INHERIT_NETWORK => {
                builder.inherit_network();
            }
            ALLOW_IP_NAME_LOOKUP => {
                builder.allow_ip_name_lookup(true);
            }
            _ => return Err(anyhow::anyhow!("Invalid capability")),
        };
    }
    Ok(builder)
}

fn add_builder_mounts(
    mut builder: WasiCtxBuilder,
    mounts: Vec<PluginMount>,
) -> Result<WasiCtxBuilder> {
    for mount in mounts.into_iter() {
        // builder.preopened_dir(
        //     mount.host_path,
        //     mount.guest_path,
        //     mount.dir_perms,
        //     mount.file_perms,
        // );
    }
    Ok(builder)
}

pub fn create_store(engine: &Engine, cloud_module: CloudModule) -> Result<Store<MyState>> {
    let mut builder = WasiCtxBuilder::new();
    builder = add_builder_capabilities(builder, cloud_module.capabilities)?;
    builder = add_builder_mounts(builder, cloud_module.mounts)?;

    Ok(Store::new(
        &engine,
        MyState {
            ctx: builder.build(),
            table: ResourceTable::default(),
        },
    ))
}
