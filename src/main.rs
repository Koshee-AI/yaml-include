use std::{fs::write, path::PathBuf};

use anyhow::Result;
use clap::Parser;

/// Output yaml with processed "!include" tags
#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// main yaml file path to process
    file_path: PathBuf,
    /// optional output path (output to stdout if not set)
    #[arg(short, long)]
    output_path: Option<PathBuf>,
}

mod helpers;
mod transformer;

fn main() -> Result<()> {
    let args = Args::parse();

    let transformer = transformer::Transformer::new(args.file_path, None)?;
    let data = format!("{}", transformer);

    match args.output_path {
        Some(path) => {
            write(path, data)?;
        }
        None => print!("{}", data),
    };

    Ok(())
}
