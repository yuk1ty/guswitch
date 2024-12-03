use clap::Parser;
use guswitch::{
    command::{exec_user_switch, show_configured_users_list},
    config::{try_load_config, try_resolve_path},
    opts::{GusCommand, Opts},
};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    let cfg_path = try_resolve_path(opts.config)?;
    let cfg = try_load_config(cfg_path)?;
    match opts.command {
        Some(GusCommand::List) => show_configured_users_list(cfg),
        None => exec_user_switch(cfg, opts.local),
    }
}
