use clap::Parser;
use guswitch::{
    command::{exec_user_switch, show_configured_user, show_configured_users_list, SwitchMode},
    config::{try_load_config, try_resolve_path},
    opts::{GsuCommand, Opts},
};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    let cfg_path = try_resolve_path(opts.config)?;
    let cfg = try_load_config(cfg_path)?;
    let mode = if opts.local {
        SwitchMode::Local
    } else {
        SwitchMode::Global
    };
    match opts.command {
        Some(GsuCommand::List) => show_configured_users_list(cfg),
        Some(GsuCommand::Current) => {
            let output = show_configured_user(&mode)?;
            println!("{output}");
            Ok(())
        }
        None => exec_user_switch(cfg, &mode),
    }
}
