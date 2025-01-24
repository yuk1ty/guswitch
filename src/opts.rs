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
    pub command: GsuCommand,
}

#[derive(Subcommand)]
pub enum GsuCommand {
    #[command(alias = "s")]
    Switch {
        #[arg(short, long, default_value_t = false)]
        global: bool,
    },
    #[command(alias = "ls")]
    List,
    #[command(alias = "g")]
    Get {
        #[arg(short, long, default_value_t = false)]
        global: bool,
    },
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
