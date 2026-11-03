// crates/r0-cli/src/args.rs
use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "r0-cli", version, about = "RISC-V Soft-IP Test Harness CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Build(BuildArgs),
    Run(RunArgs),
    Kernel(KernelCommandArgs),
}

#[derive(Args, Debug)]
pub struct KernelCommandArgs {
    #[command(subcommand)]
    pub action: KernelActions,
}

#[derive(Subcommand, Debug)]
pub enum KernelActions {
    /// Inspect and audit a RISC-V kernel source tree
    Inspect {
        #[arg(short, long, default_value = "kernel/linux-riscv")]
        path: PathBuf,
    },
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg(short, long, default_value = "Config.toml")]
    pub config: String,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(short, long, default_value = "firmware.elf")]
    pub firmware: String,
    #[arg(short, long, default_value_t = 1000)]
    pub timeout_cycles: u64,
}
