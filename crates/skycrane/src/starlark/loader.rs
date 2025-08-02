use super::std::{skycrane_std, CloudModule};
use crate::wasm::plugins::load_plugin;
use crate::WasmPlugin;
use anyhow::{Context, Result};
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{Value, ValueLike};
use starlark_syntax::syntax::module::AstModuleFields;
use std::path::Path;
use tokio::fs;

pub async fn load(config_path: impl AsRef<Path>, cloud_module: CloudModule) -> Result<WasmPlugin> {
    let plugins_path = config_path.as_ref().join("plugins");

    load_plugin(config_path, &cloud_module)
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
                .unwrap_or_default()
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
        .ok_or(anyhow::anyhow!("failed to interpret CloudModule"))?;

    Ok(cloud_mod.clone())
}
