use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoadedConfiguration {
    pub users: Vec<GitUser>,
}

impl From<LoadedConfiguration> for ConfiguredGitUsers {
    fn from(config: LoadedConfiguration) -> Self {
        let cache = config
            .users
            .into_iter()
            .map(|user| (user.name, GitUserConfig { email: user.email }))
            .collect::<BTreeMap<GitUserName, GitUserConfig>>();
        ConfiguredGitUsers(cache)
    }
}

type GitUserName = String;

pub struct GitUserConfig {
    pub email: String,
}

pub struct ConfiguredGitUsers(pub BTreeMap<GitUserName, GitUserConfig>);

pub fn try_resolve_path(overriden_path: Option<PathBuf>) -> eyre::Result<PathBuf> {
    let cfg_path: PathBuf = match overriden_path {
        Some(path) => path,
        None => {
            let mut home = PathBuf::from(
                std::env::var("XDG_CONFIG_HOME")
                    .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap())),
            );
            home.push("gus");
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
        std::env::set_var("XDG_CONFIG_HOME", &xdg_config_home);

        // Act
        let cfg_path = try_resolve_path(None)?;

        // Assert
        assert_eq!(cfg_path, xdg_config_home.join("gus").join("config.toml"));

        Ok(())
    }

    #[test]
    fn should_get_home_path_when_xdg_config_home_was_not_set() -> eyre::Result<()> {
        // Arrange
        std::env::set_var("HOME", "root");

        // Act
        let cfg_path = try_resolve_path(None)?;

        // Assert
        assert_eq!(cfg_path, PathBuf::from("root/.config/gus/config.toml"));

        Ok(())
    }

    #[test]
    fn should_get_specific_path_when_overriden() -> eyre::Result<()> {
        // Arrange
        let overriden_path = PathBuf::from("/tmp/gus/config.toml");

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
        let file_path = dir.join("gus").join("config.toml");
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
