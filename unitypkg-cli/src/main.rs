mod cli;

use std::{
    fs::{create_dir_all, File},
    io::stdout,
};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands};
use unitypkg_core::{reader::read_package, unpack::unpack_package};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Completions { shell }) => {
            let cmd = Cli::command();
            generate(
                *shell,
                &mut cmd.clone(),
                cmd.get_name().to_string(),
                &mut stdout(),
            );
        }
        Some(Commands::Unpack { input, output }) => {
            for input in input {
                let package = read_package(File::open(input).unwrap())
                    .expect("Invalid unitypackage. Please check the file and try again.");

                if !output.exists() {
                    create_dir_all(&output).unwrap();
                }

                unpack_package(package, &output).expect("Failed to unpack unitypackage.");
            }
        }
        None => {}
    }
}
