use std::process::Command;

use eyre::OptionExt;

use crate::{
    config::{ConfiguredGitUsers, LoadedConfiguration},
    prompt::PromptArg,
    table::make_table,
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
            exec_switch_command(
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
            show_configured_user(&mode)?;
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

fn exec_switch_command(mode: &SwitchMode, user_name: &str, email: &str) -> eyre::Result<()> {
    Command::new("git")
        .args(["config", mode.to_arg(), "user.name", user_name])
        .output()?;
    Command::new("git")
        .args(["config", mode.to_arg(), "user.email", email])
        .output()?;
    Ok(())
}

fn show_configured_user(mode: &SwitchMode) -> eyre::Result<()> {
    let user_name_output = Command::new("git")
        .args(["config", mode.to_arg(), "user.name"])
        .output()?;
    let user_email_output = Command::new("git")
        .args(["config", mode.to_arg(), "user.email"])
        .output()?;
    println!(
        "user: {}, email: {} [{}]",
        String::from_utf8(user_name_output.stdout)?.trim(),
        String::from_utf8(user_email_output.stdout)?.trim(),
        mode.to_token(),
    );
    Ok(())
}

pub fn show_configured_users_list(cfg: LoadedConfiguration) -> eyre::Result<()> {
    let table = make_table(cfg.users);
    println!("{table}");
    Ok(())
}
