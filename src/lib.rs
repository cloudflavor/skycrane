use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
