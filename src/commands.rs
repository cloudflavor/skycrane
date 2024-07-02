use crate::{Cli, WasmPlugin};
use anyhow::Result;

pub async fn init(opts: &Cli, _plugin: WasmPlugin) -> Result<()> {
    println!("{:?}", opts);
    Ok(())
}

pub fn apply(_opts: &Cli) -> Result<()> {
    Ok(())
}

pub fn reconcile(_opts: &Cli) -> Result<()> {
    Ok(())
}

pub fn destroy(_opts: &Cli) -> Result<()> {
    Ok(())
}
