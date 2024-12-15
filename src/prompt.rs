use inquire::Select;

use crate::config::{ConfiguredGitUsers, GitUserName};

pub struct PromptArg<'a> {
    pub configured_users: ConfiguredGitUsers,
    pub select: Select<'a, GitUserName>,
}

impl PromptArg<'_> {
    pub fn new(cache: ConfiguredGitUsers) -> Self {
        let keys = cache.0.keys().cloned().collect::<Vec<GitUserName>>();
        let select = Select::new("Select a git user", keys);
        Self {
            configured_users: cache,
            select,
        }
    }
}
