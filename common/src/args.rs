use crate::*;
use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}
impl Args {
    pub fn input(&self) -> Result<String> {
        Ok(std::fs::read_to_string(&self.input_file)?)
    }
}
