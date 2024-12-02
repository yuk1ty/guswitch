use clap::Parser;
use guswitch::{
    command::{exec_configured_users, exec_user_switch},
    config::{try_load_config, try_resolve_path},
    opts::{GusCommand, Opts},
};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    let cfg_path = try_resolve_path(opts.config)?;
    let cfg = try_load_config(cfg_path)?;
    match opts.command {
        Some(GusCommand::List) => exec_configured_users(cfg),
        None => exec_user_switch(cfg, opts.local),
    }
}
