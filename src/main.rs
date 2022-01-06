mod commands;
mod entry;
mod model;

use structopt::StructOpt;
use std::io;
use std::path::{Path, PathBuf};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let input_dir = Path::new(&args.input);
    let output_dir = Path::new(&args.output);

    let _ = commands::build(input_dir, output_dir);
    Ok(())
}
