use crate::wasm::plugins::load_plugin;
use crate::WasmPlugin;
use allocative::Allocative;
use anyhow::{Context, Result};
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::AllocValue;
use starlark::values::{Heap, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueLike};
use starlark::{starlark_module, starlark_simple_value};
use starlark_derive::starlark_value;
use starlark_syntax::syntax::module::AstModuleFields;
use std::fmt;
use std::path::Path;
use tokio::fs;

pub async fn load(config_path: impl AsRef<Path>, module_name: &str) -> Result<WasmPlugin> {
    let plugins_path = config_path.as_ref().join("plugins");

    load_plugin(config_path, module_name)
        .await
        .with_context(|| format!("Failed to load plugin from {:?}", plugins_path))
}

pub async fn read_config(path: impl AsRef<Path>) -> Result<String> {
    let mut entries = fs::read_dir(&path).await?;
    let mut star_files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file()
            && entry
                .file_name()
                .to_str()
                .map(|f| f.ends_with(".star"))
                .unwrap_or(false)
        {
            let mut data = fs::read(entry.path()).await?;
            star_files.append(&mut data);
        }
    }

    String::from_utf8(star_files).with_context(|| {
        format!(
            "Failed to parse starlark file at {:?} as UTF-8",
            path.as_ref()
        )
    })
}

pub fn extract_plugin(config: &str) -> Result<CloudModule> {
    let globals = GlobalsBuilder::new().with(skycrane_std).build();
    let module = Module::new();
    module.set("true", Value::new_bool(true));
    module.set("false", Value::new_bool(false));

    let mut eval = Evaluator::new(&module);

    let raw_ast = AstModule::parse("module.star", config.to_string(), &Dialect::Standard).unwrap();
    let codemap = raw_ast.codemap();
    let filtered_script = raw_ast
        .stmt_locations()
        .into_iter()
        .filter_map(|file_span| {
            let snippet = codemap.source_span(file_span.span);
            // TODO: this should be a hash map of statements
            // snippet.contains("module")
            if snippet.contains("module") {
                Some(snippet)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let filtered_ast =
        AstModule::parse("module.star", filtered_script, &Dialect::Standard).unwrap();

    let res = eval.eval_module(filtered_ast, &globals).unwrap();
    let cloud_mod = res
        .downcast_ref::<CloudModule>()
        .ok_or(anyhow::anyhow!("Failed to downcast CloudModule"))?;

    Ok(cloud_mod.clone())
}

#[starlark_module]
fn skycrane_std(builder: &mut GlobalsBuilder) {
    const INHERIT_ARGS: String = String::from("InheritArgs");
    const INHERIT_ENV: String = String::from("InheritEnv");
    const INHERIT_STDIO: String = String::from("InheritStdio");
    const INHERIT_STDOUT: String = String::from("InheritStdout");

    fn mount(
        host_path: String,
        guest_path: String,
        read: bool,
        write: bool,
    ) -> anyhow::Result<i32> {
        println!("{host_path} - {guest_path} - {read} - {write}");
        Ok(1)
    }

    fn module<'v>(
        name: String,
        version: String,
        capabilities: Option<Value<'v>>,
        mounts: Option<Value<'v>>,
        heap: &'v Heap,
    ) -> anyhow::Result<CloudModule> {
        if let Some(mounts) = mounts {
            for mount in mounts.iterate(heap).unwrap() {
                let a = mount
                    .downcast_ref::<PluginMount>()
                    .with_context(|| "failed to interpret mount declaration");
                println!("{:?}", a);
            }
        }

        if let Some(caps) = capabilities {
            for item in caps.iterate(heap).unwrap() {}
        }

        Ok(CloudModule {
            name,
            version,
            capabilities: Vec::new(),
            mounts: Vec::new(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub enum PluginCaps {
    InheritArgs,
    InheritEnv,
    InheritStdio,
    InheritStdout,
}

impl<'v> AllocValue<'v> for PluginCaps {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        match self {
            PluginCaps::InheritArgs => heap.alloc("InheritArgs"),
            PluginCaps::InheritStdio => heap.alloc("InheritStdIo"),
            PluginCaps::InheritStdout => heap.alloc("InheritStdout"),
            PluginCaps::InheritEnv => heap.alloc("InheritEnv"),
        }
    }
}

impl fmt::Display for PluginCaps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginCaps::InheritArgs => write!(f, "InheritArgs"),
            PluginCaps::InheritEnv => write!(f, "InheritEnv"),
            PluginCaps::InheritStdio => write!(f, "InheritStdio"),
            PluginCaps::InheritStdout => write!(f, "InheritStdout"),
        }
    }
}

#[starlark_value(type = "plugin_caps")]
impl<'v> StarlarkValue<'v> for PluginCaps {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct PluginDirPerms {
    pub read: bool,
    pub write: bool,
}

impl fmt::Display for PluginDirPerms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "read: {}, write: {}", self.read, self.write)
    }
}

#[starlark_value(type = "dir_perms")]
impl<'v> StarlarkValue<'v> for PluginDirPerms {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct PluginFilePerms {
    pub read: bool,
    pub write: bool,
}

impl fmt::Display for PluginFilePerms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "read: {}, write: {}", self.read, self.write)
    }
}

#[starlark_value(type = "file_perms")]
impl<'v> StarlarkValue<'v> for PluginFilePerms {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct PluginMount {
    pub host_path: String,
    pub guest_path: String,
    pub perms_type: String,
    pub dir_perms: PluginDirPerms,
    pub file_perms: PluginFilePerms,
}

impl<'v> AllocValue<'v> for PluginMount {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc(self)
    }
}

impl fmt::Display for PluginMount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host_path: {}, guest_path: {}, dir_perms: {}, file_perms: {}",
            self.host_path, self.guest_path, self.dir_perms, self.file_perms
        )
    }
}

#[starlark_value(type = "mount_path")]
impl<'v> StarlarkValue<'v> for PluginMount {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct CloudModule {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub mounts: Vec<PluginMount>,
}
starlark_simple_value!(CloudModule);

#[starlark_value(type = "cloud_module")]
impl<'v> StarlarkValue<'v> for CloudModule {}

impl fmt::Display for CloudModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, version: {}", self.name, self.version)
    }
}
