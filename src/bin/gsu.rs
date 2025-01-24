use clap::Parser;
use guswitch::{
    command::{exec_user_switch, show_configured_user, show_configured_users_list},
    config::{try_load_config, try_resolve_path},
    opts::{GsuCommand, Opts},
};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    let cfg_path = try_resolve_path(opts.config)?;
    let cfg = try_load_config(cfg_path)?;
    match opts.command {
        GsuCommand::List => show_configured_users_list(cfg),
        GsuCommand::Get { global } => {
            let output = show_configured_user(&global.into())?;
            println!("{output}");
            Ok(())
        }
        GsuCommand::Switch { global } => exec_user_switch(cfg, &global.into()),
    }
}
