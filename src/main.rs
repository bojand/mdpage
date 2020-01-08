#[macro_use]
extern crate log;

use std::fs::File;
use std::path::PathBuf;

use env_logger::Env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Generate simple documentation
struct Args {
    /// Title of the document
    #[structopt(long)]
    title: Option<String>,

    /// Subtitle of the document
    #[structopt(long)]
    subtitle: Option<String>,

    /// Generate full page documentation
    #[structopt(long, takes_value = false)]
    full_page: bool,

    /// Path for the directory containing data
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("warn")).init();

    let opt = Args::from_args();

    let root = opt.path.as_path();

    let initial = mdpage::Data {
        title: opt.title,
        subtitle: opt.subtitle,
        full_page: Some(opt.full_page),
        ..mdpage::Data::default()
    };

    let data = mdpage::build(root, Some(initial))?;
    debug!("{}", serde_json::to_string(&data).expect("failed to json"));

    let mut path = root.to_path_buf();
    path.push("index.html");
    let f = File::create(path)?;

    mdpage::write_data(f, &data)
}
