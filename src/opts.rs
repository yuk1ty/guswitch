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
    pub command: Option<GsuCommand>,
}

#[derive(Subcommand)]
pub enum GsuCommand {
    List,
    Current,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Opts;

    #[test]
    fn debug_opts() {
        Opts::command().debug_assert();
    }
}
