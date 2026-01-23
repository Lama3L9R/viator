mod lua;
mod build;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use mlua::Lua;

#[derive(Parser, Debug)]
#[command(name = "Viator", version, about)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Specify viator file or directory
    #[arg(short = 'f', long, value_name = "FILE_OR_DIR")]
    viator_file: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run build targets
    Build {
        /// The target you wish to execute
        target: Option<String>,
    }
}

pub struct ViatorState {
    pub cli_args: CliArgs,
    pub lua: Lua,
}

fn main() {
    let mut state = ViatorState {
        cli_args: CliArgs::parse(),
        lua: Lua::new(),
    };

}
