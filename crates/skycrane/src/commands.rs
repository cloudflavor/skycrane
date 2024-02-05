use crate::Cli;
use anyhow::Result;
use wasmtime::Instance;

pub async fn init(_opts: &Cli, _instance: Instance) -> Result<()> {
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
