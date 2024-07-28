use crate::wasm::plugins::load_plugin;
use crate::WasmPlugin;
use allocative::Allocative;
use anyhow::{Context, Result};
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{NoSerialize, ProvidesStaticType};
use starlark::values::{StarlarkValue, ValueLike};
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
    let globals = GlobalsBuilder::new().with(starlark_module).build();
    let module = Module::new();
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
fn starlark_module(builder: &mut GlobalsBuilder) {
    fn module(name: String, version: String) -> starlark::Result<CloudModule> {
        Ok(CloudModule { name, version })
    }
}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct CloudModule {
    pub name: String,
    pub version: String,
}
starlark_simple_value!(CloudModule);

impl fmt::Display for CloudModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, version: {}", self.name, self.version)
    }
}

#[starlark_value(type = "CloudModule")]
impl<'v> StarlarkValue<'v> for CloudModule {}
