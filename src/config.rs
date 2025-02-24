use std::{
    collections::BTreeMap,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use eyre::eyre;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitUser {
    /// The user name.
    pub name: String,
    /// The email address of the user.
    pub email: String,
    /// The description of the user.
    /// Note that description is only shown in listing up command.
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoadedConfiguration {
    pub users: Vec<GitUser>,
}

impl TryFrom<LoadedConfiguration> for ConfiguredGitUsers {
    type Error = eyre::Error;
    fn try_from(config: LoadedConfiguration) -> Result<Self, Self::Error> {
        let mut configured = BTreeMap::new();
        for user in config.users {
            configured.insert(user.name.try_into()?, user.email.try_into()?);
        }
        Ok(ConfiguredGitUsers(configured))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitUserName(pub String);

impl TryFrom<String> for GitUserName {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(eyre!("User name must not be empty"))
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for GitUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct GitUserEmailAddress(pub String);

impl TryFrom<String> for GitUserEmailAddress {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(eyre!("Email must not be empty"))
        } else {
            Ok(Self(value))
        }
    }
}

/// Represents a collection of users that are actually used in switching users.
pub struct ConfiguredGitUsers(pub BTreeMap<GitUserName, GitUserEmailAddress>);

/// This function try_resolve_path attempts to resolve a configuration file path.
/// If an overridden path is provided, it uses that path.
/// Otherwise, it constructs a default path based on the XDG_CONFIG_HOME environment variable or
/// defaults to $HOME/.config/gsu/config.toml.
/// It returns the resolved path wrapped in an eyre::Result.
pub fn try_resolve_path(overriden_path: Option<PathBuf>) -> eyre::Result<PathBuf> {
    let cfg_path: PathBuf = match overriden_path {
        Some(path) => path,
        None => {
            let mut home = PathBuf::from(
                std::env::var("XDG_CONFIG_HOME")
                    .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap())),
            );
            home.push("gsu");
            home.push("config.toml");
            home
        }
    };
    Ok(cfg_path)
}

pub fn try_load_config(path: impl AsRef<Path>) -> eyre::Result<LoadedConfiguration> {
    let file = fs::read_to_string(&path)?;
    Ok(toml::from_str(&file)?)
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::PathBuf;
    use std::{env::temp_dir, fs::File};

    use super::{try_load_config, try_resolve_path};

    #[test]
    fn should_get_path_from_env_when_xdg_config_home_was_set() -> eyre::Result<()> {
        // Arrange
        let xdg_config_home = PathBuf::from("$HOME/.config");
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { std::env::set_var("XDG_CONFIG_HOME", &xdg_config_home) };

        // Act
        let cfg_path = try_resolve_path(None)?;

        // Assert
        assert_eq!(cfg_path, xdg_config_home.join("gsu").join("config.toml"));

        Ok(())
    }

    #[test]
    fn should_get_home_path_when_xdg_config_home_was_not_set() -> eyre::Result<()> {
        // Arrange
        let actual_home = std::env::var("HOME")?;

        // Act
        let cfg_path = try_resolve_path(None)?;

        // Assert
        assert_eq!(
            cfg_path,
            PathBuf::from(format!("{actual_home}/.config/gsu/config.toml"))
        );

        Ok(())
    }

    #[test]
    fn should_get_specific_path_when_overriden() -> eyre::Result<()> {
        // Arrange
        let overriden_path = PathBuf::from("/tmp/gsu/config.toml");

        // Act
        let cfg_path = try_resolve_path(Some(overriden_path.clone()))?;

        // Assert
        assert_eq!(cfg_path, overriden_path);

        Ok(())
    }

    #[test]
    fn should_load_and_parse_to_specific_data_structure() -> eyre::Result<()> {
        // Arrange
        let dir = temp_dir();
        let file_path = dir.join("config.toml");
        let mut file = File::create(&file_path)?;
        writeln!(
            file,
            r#"
            [[users]]
            name = "Alice"
            email = "alice.dummy@dummydummy.com"
            description = "Alice's description"
            "#
        )?;

        // Act
        let loaded = try_load_config(&file_path)?;

        // Assert
        let actual = loaded.users.first().unwrap();
        assert_eq!(actual.name, "Alice");
        assert_eq!(actual.email, "alice.dummy@dummydummy.com");
        assert_eq!(actual.description, Some("Alice's description".into()));

        Ok(())
    }
}
