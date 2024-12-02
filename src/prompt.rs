use inquire::Select;

use crate::config::ConfiguredGitUsers;

pub struct PromptArg<'a> {
    pub cache: ConfiguredGitUsers,
    pub select: Select<'a, String>,
}

impl PromptArg<'_> {
    pub fn new(cache: ConfiguredGitUsers) -> Self {
        let keys = cache.0.keys().cloned().collect::<Vec<String>>();
        let select = Select::new("Select a git user", keys);
        Self { cache, select }
    }
}
