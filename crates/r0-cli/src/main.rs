// crates/r0-cli/src/main.rs
mod args;
mod commands {
    pub mod build;
    pub mod run;
    pub mod kernel;
}

use clap::Parser;
use args::{Cli, Commands, KernelActions};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => {
            commands::build::execute(&args)?;
        }
        Commands::Run(args) => {
            commands::run::execute(&args)?;
        }
        Commands::Kernel(args) => {
            match args.action {
                KernelActions::Inspect { path } => {
                    commands::kernel::execute_inspect(path)?;
                }
            }
        }
    }

    Ok(())
}
