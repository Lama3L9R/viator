mod lua;
mod build;
mod utils;
mod logging;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use cve_rs::null_mut;
use mlua::Lua;
use crate::build::ViatorState;

#[derive(Parser, Debug)]
#[command(name = "Viator", version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Specify viator file or directory
    #[arg(short = 'f', long, value_name = "FILE_OR_DIR", default_value = "Viator")]
    viator_file: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run build targets
    Build {
        /// The target you wish to execute
        target: Option<String>,
    },

    ExecuteLua {
        /// Execute a lua file with embedded lua interpreter
        code: String,
        /// Should we include viator environment (The V Helper)
        #[arg(long)]
        env: bool,
    },

    Version,

    Crash
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = ViatorState::create(CliArgs::parse())?;

    match &state.cli_args.command {
        Some(Commands::Build { target }) => {}
        Some(Commands::Crash) => {
            let not_null = null_mut::<u8>();
            *not_null = 0;
        }
        Some(Commands::ExecuteLua { code, env }) => {
            if *env {
                state.exec_lua_code(code.into()).inspect_err(|err| println!("{}", err));
            } else {
                let env = Lua::new();
                env.load(code).eval::<()>().inspect_err(|err| println!("{}", err));
            }
        }
        Some(Commands::Version) => {
            print!("Viator: {} ({}, {}", built_info::PKG_VERSION, built_info::RUSTC_VERSION, built_info::HOST);
            if built_info::GIT_DIRTY.is_none() || built_info::GIT_DIRTY.unwrap() {
                println!(", {} dirty)", built_info::GIT_COMMIT_HASH_SHORT.unwrap_or("unknown"))
            } else {
                println!(", {})", built_info::GIT_COMMIT_HASH_SHORT.unwrap_or("unknown"))
            }

            println!("Lua Backend: {}", state.get_lua_version());
            println!("Dependencies:");
            for (dep, ver) in built_info::DIRECT_DEPENDENCIES {
                println!("  {}: {}", dep, ver);
            }
        }

        _ => {}
    }

    Ok(())
}
