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
    /// Generate completions for your shell.
    Completions {
        /// The target shell to generate completions for.
        #[arg(value_enum, required = true)]
        shell: Shell,
    },
    /// Unpack (multiple) unitypackages into a directory.
    Unpack {
        /// The input file(s) to be processed.
        #[arg(short, long, required = true)]
        input: Vec<PathBuf>,

        /// The output directory to write all processed inputs to.
        output: PathBuf,
    },
}
