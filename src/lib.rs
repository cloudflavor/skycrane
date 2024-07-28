mod commands;
mod config;
mod wasm;

pub use commands::init;
pub use wasm::engine::init_plugin;
pub use wasm::plugins::{load_plugin, WasmPlugin};
use wasmtime::component::bindgen;

use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

bindgen!({
    path: "wit",
});

#[derive(StructOpt, Clone, Debug)]
pub struct Cli {
    /// The path to the configuration directory, this is where skycrane keeps its main config as well
    /// as the modules directory.
    #[structopt(short, long, default_value = "~/.config/skycrane", parse(from_str = expand_path), env = "SKYCRANE_CONFIG_PATH")]
    pub config_path: PathBuf,
    #[structopt(subcommand)]
    pub commands: Commands,
    /// The log level to use, available levels are: trace, debug, info, warn, error
    #[structopt(
        long = "log-level",
        default_value = "info",
        possible_values = &["trace", "debug", "info", "warn", "error"],
        env = "SKYCRANE_LOG_LEVEL"
    )]
    pub log_level: String,
}

#[derive(StructOpt, Clone, Debug)]
pub enum Commands {
    Init(Init),
    Plan(Plan),
    Apply(Apply),
    Reconcile(Reconcile),
    Destroy(Destroy),
}

#[derive(StructOpt, Clone, Debug)]
pub struct Apply {}

/// Initialize a new repository at the given path This will download the specified plugin
/// in the skycrane plugins config directory.
#[derive(StructOpt, Clone, Debug)]
pub struct Init {
    #[structopt(parse(from_os_str), help = "Path to initialize")]
    pub path: PathBuf,
}

#[derive(StructOpt, Clone, Debug)]
pub struct Plan {
    #[structopt(parse(from_os_str), help = "Path to plan")]
    pub output: PathBuf,
}

#[derive(StructOpt, Clone, Debug)]
pub struct Reconcile {}

#[derive(StructOpt, Clone, Debug)]
pub struct Destroy {}

fn expand_path(path_str: &str) -> PathBuf {
    if path_str.starts_with('~') {
        let home_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        if let Some(home) = env::var_os(home_var) {
            let without_tilde = &path_str.strip_prefix('~').unwrap();
            let normalized_path = without_tilde
                .trim_start_matches('/')
                .trim_start_matches('\\')
                .replace('\\', "/"); // Normalize path separators
            return PathBuf::from(home).join(normalized_path);
        }
    }
    PathBuf::from(path_str)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_expand_path() {
        let home_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        let home = env::var_os(home_var).unwrap();
        let home_str = home.to_str().unwrap();

        assert_eq!(
            expand_path("~/foo/bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~\\foo\\bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~/foo\\bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~\\foo/bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~/.config/skycrane"),
            PathBuf::from(format!("{}/.config/skycrane", home_str))
        );
        assert_eq!(
            expand_path("~/.config\\skycrane"),
            PathBuf::from(format!("{}/.config/skycrane", home_str))
        );
    }
}
