use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Opts {
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    pub local: bool,
    #[command(subcommand)]
    pub command: Option<GusCommand>,
}

#[derive(Subcommand)]
pub enum GusCommand {
    List,
}
