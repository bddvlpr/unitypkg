use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Completions {
        #[arg(value_enum, required = true)]
        shell: Shell,
    },
    Unpack {
        #[arg(short, long, required = true)]
        input: Vec<PathBuf>,

        output: PathBuf,
    },
}
