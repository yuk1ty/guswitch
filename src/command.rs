use std::process::Command;

use eyre::OptionExt;
use tabled::{settings::Style, Table};

use crate::{
    config::{ConfiguredGitUsers, LoadedConfiguration},
    prompt::PromptArg,
    table::TabledGitUser,
};

pub fn exec_user_switch(cfg: LoadedConfiguration, local: bool) -> eyre::Result<()> {
    let users: ConfiguredGitUsers = cfg.into();
    let mode = if local {
        SwitchMode::Local
    } else {
        SwitchMode::Global
    };
    let prompt_arg = PromptArg::new(users);
    let ans = prompt_arg.select.prompt();
    match ans {
        Ok(choice) => {
            execute_git_command(
                &mode,
                &choice,
                prompt_arg
                    .cache
                    .0
                    .get(&choice)
                    .ok_or_eyre("email not found")?
                    .email
                    .as_str(),
            )?;
            println!("Switched {} git user to {}", mode.to_token(), choice);
        }
        Err(_) => println!("Please try again"),
    }
    Ok(())
}

enum SwitchMode {
    Global,
    Local,
}

impl SwitchMode {
    fn to_arg(&self) -> &str {
        match self {
            Self::Global => "--global",
            Self::Local => "--local",
        }
    }

    fn to_token(&self) -> &str {
        match self {
            Self::Global => "global",
            Self::Local => "local",
        }
    }
}

fn execute_git_command(mode: &SwitchMode, user_name: &str, email: &str) -> eyre::Result<()> {
    Command::new("git")
        .args(["config", mode.to_arg(), "user.name", user_name])
        .output()?;
    Command::new("git")
        .args(["config", mode.to_arg(), "user.email", email])
        .output()?;
    Ok(())
}

pub fn exec_configured_users(cfg: LoadedConfiguration) -> eyre::Result<()> {
    let tabled_users: Vec<TabledGitUser> = cfg.users.into_iter().map(TabledGitUser::from).collect();
    let mut table = Table::new(tabled_users);
    table.with(Style::modern_rounded());
    println!("{table}");
    Ok(())
}
