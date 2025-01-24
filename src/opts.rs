use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Opts {
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: GsuCommand,
}

/// The subcommands of the `gsu` command.
/// `get` accpepts `get --local` or `get --global` to show the current user.
/// `switch` accepts `switch --local` or `switch --global` to switch users.
/// The above two can omit flags to use the default value.
/// The flagging strategy is intentionally aligned with the `git config` command.
/// `list` lists up all the configured users.
#[derive(Subcommand)]
pub enum GsuCommand {
    #[command(alias = "s")]
    Switch {
        #[arg(short, long, default_value_t = true)]
        local: bool,
        #[arg(short, long, default_value_t = false, conflicts_with = "local")]
        global: bool,
    },
    #[command(alias = "ls")]
    List,
    #[command(alias = "g")]
    Get {
        #[arg(short, long, default_value_t = true)]
        local: bool,
        #[arg(short, long, default_value_t = false, conflicts_with = "local")]
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
