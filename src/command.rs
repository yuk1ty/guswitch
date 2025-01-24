use std::process::Command;

use eyre::OptionExt;

use crate::{
    config::{ConfiguredGitUsers, GitUserEmailAddress, GitUserName, LoadedConfiguration},
    prompt::PromptArg,
    table::make_table,
};

/// Switching git user according to the target. When `local` is `true`, it switches the local git user.
/// Otherwise global .gitconfig is updated.
pub fn exec_user_switch(cfg: LoadedConfiguration, mode: &SwitchMode) -> eyre::Result<()> {
    let users: ConfiguredGitUsers = cfg.try_into()?;
    let prompt_arg = PromptArg::new(users);
    let ans = prompt_arg.select.prompt();
    match ans {
        Ok(choice) => {
            exec_switch_command(
                &mode,
                &choice,
                prompt_arg
                    .configured_users
                    .0
                    .get(&choice)
                    .ok_or_eyre("email not found")?,
            )?;
            let output = show_configured_user(&mode)?;
            println!("{output}");
        }
        Err(e) => eprintln!("{e}"),
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum SwitchMode {
    Global,
    Local,
}

impl SwitchMode {
    /// If the global flag is turned on, it will prior to the local flag.
    /// FIXME: This implementation is kind of like counterintuitive. Have to improve this.
    pub fn new(_local: bool, global: bool) -> Self {
        if global {
            Self::Global
        } else {
            Self::Local
        }
    }

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

fn exec_switch_command(
    mode: &SwitchMode,
    user_name: &GitUserName,
    email: &GitUserEmailAddress,
) -> eyre::Result<()> {
    Command::new("git")
        .args(["config", mode.to_arg(), "user.name", user_name.0.as_str()])
        .output()?;
    Command::new("git")
        .args(["config", mode.to_arg(), "user.email", email.0.as_str()])
        .output()?;
    Ok(())
}

pub fn show_configured_user(mode: &SwitchMode) -> eyre::Result<String> {
    let user_name_output = Command::new("git")
        .args(["config", mode.to_arg(), "user.name"])
        .output()?;
    let user_email_output = Command::new("git")
        .args(["config", mode.to_arg(), "user.email"])
        .output()?;
    Ok(format!(
        "user: {}, email: {} [{}]",
        String::from_utf8(user_name_output.stdout)?.trim(),
        String::from_utf8(user_email_output.stdout)?.trim(),
        mode.to_token(),
    ))
}

/// Show users in the configuration file in a table format.
pub fn show_configured_users_list(cfg: LoadedConfiguration) -> eyre::Result<()> {
    let table = make_table(cfg.users);
    println!("{table}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::SwitchMode;

    #[test]
    fn test_switch_mode_creation() {
        assert_eq!(SwitchMode::new(false, false), SwitchMode::Local);
        assert_eq!(SwitchMode::new(true, false), SwitchMode::Local);
        assert_eq!(SwitchMode::new(false, true), SwitchMode::Global);
        assert_eq!(SwitchMode::new(true, true), SwitchMode::Global);
    }
}
