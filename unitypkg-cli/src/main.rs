use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use clap::Parser;
use unitypkg_core::{reader::read_package, unpack::unpack_package};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    for input in cli.input {
        let package = read_package(File::open(input).unwrap());

        if !cli.output.exists() {
            create_dir_all(&cli.output).unwrap();
        }

        unpack_package(package, &cli.output);
    }
}
