#[macro_use]
extern crate log;

use std::path::PathBuf;
use std::fs::File;

use env_logger::Env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Generate simple documentation
struct Args {
    /// Path for the directory contianing data
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("warn")).init();

    let opt = Args::from_args();
    
    let root = opt.path.as_path();
    let data = mdpage::build(root)?;
    debug!("{}", serde_json::to_string(&data).expect("failed to json"));

    let mut path = root.to_path_buf();
    path.push("index.html");
    let f = File::create(path)?;

    mdpage::write_data(f, &data).expect("failed to write data");
    
    return Ok(());
}
