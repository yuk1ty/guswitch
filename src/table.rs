use tabled::Tabled;

use crate::config::GitUser;

#[derive(Tabled)]
pub struct TabledGitUser {
    pub name: String,
    pub email: String,
    pub description: String, // when the original string is None, should be empty string
}

impl From<GitUser> for TabledGitUser {
    fn from(value: GitUser) -> Self {
        TabledGitUser {
            name: value.name,
            email: value.email,
            description: value.description.unwrap_or_default(),
        }
    }
}
