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
